use std::process::{Command, Stdio};
use std::error::Error;

pub async fn start_local_stellar() -> Result<(), Box<dyn Error>> {
    let status = Command::new("stellar")
        .arg("network")
        .arg("container")
        .arg("start")
        .arg("local")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
    else {
        // Check for the specific exit code that indicates a conflict (container already running)
        if let Some(1) = status.code() {
            eprintln!("Container is already running, proceeding to health check...");
        } else {
            return Err(format!("Command failed with status: {status}").into());
        }
    }

    wait_for_stellar_health().await?;
    Ok(())
}

async fn wait_for_stellar_health() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    loop {
        let res = client.post("http://localhost:8000/rpc")
            .header("Content-Type", "application/json")
            .body(r#"{"jsonrpc": "2.0", "id": 1, "method": "getHealth"}"#)
            .send()
            .await?;

        if res.status().is_success() {
            let health_status: serde_json::Value = res.json().await?;
            if health_status["result"]["status"] == "healthy" {
                break;
            } 
            eprintln!("Stellar status is not healthy: {health_status:?}");
        } else {
            eprintln!("Health check request failed with status: {}", res.status());
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        eprintln!("Retrying health check.");
    }
    Ok(())
}