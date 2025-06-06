// src/clients/near.rs
use async_trait::async_trait;
use crate::clients::NodeClient;
use std::process::Command;
use tracing::info;

pub struct NearClient {
    service_name: String,
}

impl NearClient {
    pub async fn new() -> Self {
        Self {
            service_name: "neard.service".to_string(),
        }
    }

    fn check_systemctl_status(&self) -> Result<String, String> {
        let output = Command::new("systemctl")
            .arg("is-active")
            .arg(&self.service_name)
            .output()
            .map_err(|e| format!("Failed to call systemctl: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    fn restart_service(&self) -> Result<(), String> {
        let status = Command::new("systemctl")
            .arg("restart")
            .arg(&self.service_name)
            .status()
            .map_err(|e| format!("Restart failed: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err("Restart command returned a failure status".to_string())
        }
    }
}

#[async_trait]
impl NodeClient for NearClient {
    fn name(&self) -> &'static str {
        "near"
    }

    async fn is_healthy(&self) -> bool {
        matches!(self.check_systemctl_status(), Ok(s) if s == "active")
    }

    async fn fetch_log_tail(&self, lines: usize) -> Vec<String> {
        let output = Command::new("journalctl")
            .arg("-u")
            .arg(&self.service_name)
            .arg("--no-pager")
            .arg("--lines")
            .arg(lines.to_string())
            .output();

        match output {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout).to_string();
                text.lines().map(|l| l.to_string()).collect()
            }
            Err(_) => vec!["Error reading logs".to_string()],
        }
    }

    async fn restart(&self) -> Result<(), String> {
        self.restart_service()
    }
}
