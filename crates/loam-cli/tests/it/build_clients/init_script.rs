use crate::util::{find_binary, TestEnv};

#[test]
fn build_command_runs_init() {
    TestEnv::from("soroban-init-boilerplate", |env| {
        env.set_environments_toml(
            r#"
development.accounts = [
{ name = "alice" },
{ name = "bob" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.client = false
soroban_increment_contract.client = false
soroban_custom_types_contract.client = false
soroban_auth_contract.client = false

[development.contracts.soroban_token_contract]
client = true
init = """
initialize --symbol ABND --decimal 7 --name abundance --admin alice
mint --amount 2000000 --to alice
"""
"#,
        );

        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        // ensure the invoke commands are run with the proper source account
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains(" -- initialize --symbol ABND --decimal 7 --name abundance --admin alice"));
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains(" -- mint --amount 2000000 --to alice"));
        assert!(String::from_utf8_lossy(&output.stderr).contains(
            "✅ Initialization script for \"soroban_token_contract\" completed successfully"
        ));
        // ensure setting STELLAR_ACCOUNT works
        env.set_environments_toml(
            r#"
development.accounts = [
{ name = "alice" },
{ name = "bob" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.client = false
soroban_increment_contract.client = false
soroban_custom_types_contract.client = false
soroban_auth_contract.client = false

[development.contracts.soroban_token_contract]
client = true
init = """
STELLAR_ACCOUNT=bob initialize --symbol ABND --decimal 7 --name abundance --admin bob 
STELLAR_ACCOUNT=bob mint --amount 2000000 --to bob 
"""
"#,
        );
        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        // ensure the invoke commands are run with the proper source account
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("--source-account bob -- initialize --symbol ABND --decimal 7 --name abundance --admin bob"));
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("--source-account bob -- mint --amount 2000000 --to bob"));
        assert!(String::from_utf8_lossy(&output.stderr).contains(
            "✅ Initialization script for \"soroban_token_contract\" completed successfully"
        ));
    });
}

#[test]
fn init_handles_quotations_and_subcommands_in_script() {
    TestEnv::from("soroban-init-boilerplate", |env| {
        let binary_path =
            find_binary("stellar").expect("Stellar binary not found. Test cannot proceed.");

        let binary_path_str = binary_path.to_string_lossy();
        env.set_environments_toml(format!(
            r#"
    development.accounts = [
    {{ name = "me" }},
    ]

    [development.network]
    rpc-url = "http://localhost:8000/rpc"
    network-passphrase = "Standalone Network ; February 2017"

    [development.contracts]
    hello_world.client = false
    soroban_increment_contract.client = false
    soroban_auth_contract.client = false
    soroban_token_contract.client = false

    [development.contracts.soroban_custom_types_contract]
    client = true
    init = """
    test_init --resolution 300000 --assets '[{{"Stellar": "$({binary_path_str} contract id asset --asset native)"}} ]' --decimals 14 --base '{{"Stellar":"$({binary_path_str} contract id asset --asset native)"}}'
    """
    "#
        ));

        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        // Ensure the command executed successfully
        assert!(output.status.success());

        // Check for the presence of the initialization commands in the output
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(" -- test_init --resolution 300000")
        );

        // Check for successful initialization message
        assert!(String::from_utf8_lossy(&output.stderr).contains(
            "✅ Initialization script for \"soroban_custom_types_contract\" completed successfully"
        ));
    });
}

#[test]
fn init_scripts_run_in_specified_order() {
    TestEnv::from("soroban-init-boilerplate", |env| {
        let binary_path =
            find_binary("stellar").expect("Stellar binary not found. Test cannot proceed.");
        let binary_path_str = binary_path.to_string_lossy();
        // First configuration: custom_types then token
        env.set_environments_toml(format!(
            r#"
development.accounts = [
{{ name = "alice" }},
{{ name = "bob" }},
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.client = false
soroban_increment_contract.client = false
soroban_auth_contract.client = false

[development.contracts.soroban_custom_types_contract]
client = true
init = """
test_init --resolution 300000 --assets '[{{"Stellar": "$({binary_path_str} contract id asset --asset native)"}} ]' --decimals 14 --base '{{"Stellar":"$({binary_path_str} contract id asset --asset native)"}}'
"""

[development.contracts.soroban_token_contract]
client = true
init = """
STELLAR_ACCOUNT=bob initialize --symbol ABND --decimal 7 --name abundance --admin bob 
STELLAR_ACCOUNT=bob mint --amount 2000000 --to bob 
"""
"#
        ));

        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success());

        // Check order of initialization
        let custom_types_index = stderr
            .find("Running initialization script for \"soroban_custom")
            .expect("Custom types init not found");
        let token_index = stderr
            .find("Running initialization script for \"soroban_token")
            .expect("Token init not found");
        assert!(
            custom_types_index < token_index,
            "Custom types should be initialized before token"
        );

        // Second configuration: token then custom_types
        env.set_environments_toml(format!(
            r#"
development.accounts = [
{{ name = "alice" }},
{{ name = "bob" }},
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.client = false
soroban_increment_contract.client = false
soroban_auth_contract.client = false

[development.contracts.soroban_token_contract]
client = true
init = """
STELLAR_ACCOUNT=bob initialize --symbol ABND --decimal 7 --name abundance --admin bob 
STELLAR_ACCOUNT=bob mint --amount 2000000 --to bob 
"""

[development.contracts.soroban_custom_types_contract]
client = true
init = """
test_init --resolution 300000 --assets '[{{"Stellar": "$({binary_path_str} contract id asset --asset native)"}} ]' --decimals 14 --base '{{"Stellar":"$({binary_path_str} contract id asset --asset native)"}}'
"""
"#));

        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success());

        // Check order of initialization
        let token_index = stderr
            .find("Running initialization script for \"soroban_token")
            .expect("Token init not found");
        let custom_types_index = stderr
            .find("Running initialization script for \"soroban_custom")
            .expect("Custom types init not found");
        assert!(
            token_index < custom_types_index,
            "Token should be initialized before custom types"
        );
    });
}
