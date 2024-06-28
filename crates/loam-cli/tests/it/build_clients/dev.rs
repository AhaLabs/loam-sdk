use crate::util::TestEnv;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::{sleep, timeout};
use tokio_stream::wrappers::LinesStream; // Import this wrapper
use tokio_stream::StreamExt; // Import StreamExt trait for iterating streams

#[tokio::test]
async fn dev_command_watches_for_changes_and_environments_toml() {
    TestEnv::from_async("soroban-init-boilerplate", |env| async {
        Box::pin(async move {

            let mut dev_process = env
                .loam_process("dev")
                .current_dir(&env.cwd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to spawn dev process");

            let stderr = dev_process.stderr.take().unwrap();
            let mut stderr_lines = LinesStream::new(BufReader::new(stderr).lines());

            // Wait for initial build to complete
            wait_for_output(
                &mut stderr_lines,
                "Watching for changes. Press Ctrl+C to stop.",
            )
            .await;

            // Test 1: Modify a source file
            let file_changed = "contracts/hello_world/src/lib.rs";
            env.modify_file(file_changed, "// This is a test modification");
            let file_changed_path = env.cwd.join(file_changed);

            // Wait for the dev process to detect changes and rebuild
            wait_for_output(
                &mut stderr_lines,
                &format!("File changed: {:?}", file_changed_path),
            )
            .await;

            wait_for_output(
                &mut stderr_lines,
                "Watching for changes. Press Ctrl+C to stop.",
            )
            .await;

            // Test 2: Create and modify environments.toml
            env.set_environments_toml(
                r#"
development.accounts = [
    { name = "alice" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.workspace = true
soroban_increment_contract.workspace = true
"#,
            );

            // Wait for the dev process to detect changes and rebuild
            wait_for_output(
                &mut stderr_lines,
                "🌐 using network at http://localhost:8000/rpc",
            )
            .await;

            wait_for_output(
                &mut stderr_lines,
                "Watching for changes. Press Ctrl+C to stop.",
            )
            .await;
            env.set_environments_toml(
                r#"
development.accounts = [
    { name = "alice" },
]

[development.network]
rpc-url = "http://localhost:9000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.workspace = true
soroban_increment_contract.workspace = true
"#,
            );

            // Wait for the dev process to detect changes and rebuild
            wait_for_output(
                &mut stderr_lines,
                "🌐 using network at http://localhost:9000/rpc",
            )
            .await;
            dev_process
                .kill()
                .await
                .expect("Failed to kill dev process");
        })
        .await;
    })
    .await;
}

async fn wait_for_output<
    T: tokio_stream::Stream<Item = Result<String, tokio::io::Error>> + Unpin,
>(
    lines: &mut T,
    expected: &str,
) {
    let timeout_duration = Duration::from_secs(120); // 2 minutes
    let result = timeout(timeout_duration, async {
        while let Some(line) = lines.next().await {
            let line = line.expect("Failed to read line");
            if line.contains(expected) {
                return;
            }
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await;
    match result {
        Ok(_) => {
            println!("Found string {}", expected)
        }
        Err(_) => panic!("Timed out waiting for output: {}", expected),
    }
}
