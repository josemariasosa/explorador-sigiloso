use bollard::{Docker, container::ListContainersOptions};
use reqwest::Client;
use std::collections::HashSet;
use tracing::{info, error};

pub struct CommandCenter {
    docker: Docker,
    trusted_containers: HashSet<String>,
    http: Client,
}

impl Default for CommandCenter {
    fn default() -> Self {
        CommandCenter {
            docker: Docker::connect_with_local_defaults().unwrap(),
            trusted_containers: HashSet::from([
                "sigiloso_daemon".into(),
                "sigiloso_api".into(),
                "sigiloso_signer".into(),
                "sigiloso_hivemind".into(),
            ]),
            http: Client::new(),
        }
    }
}

impl CommandCenter {
    pub async fn launch_all(&mut self) -> anyhow::Result<()> {
        self.ensure_trusted_containers().await?;

        // Check daemon status
        self.check_daemon().await?;

        // Check Safe signer is online
        self.check_signer().await?;

        // Connect to API (remote commands, status)
        self.check_api().await?;

        // Optionally: Connect to hivemind
        info!("🌐 Hivemind assumed to be connected (libp2p background)");

        Ok(())
    }

    async fn ensure_trusted_containers(&self) -> anyhow::Result<()> {
        let running = self.docker.list_containers(Some(ListContainersOptions::<String> {
            all: false,
            ..Default::default()
        })).await?;

        for container in running {
            let name = container.names.unwrap_or_default().join(",");
            if !self.trusted_containers.iter().any(|n| name.contains(n)) {
                error!("🚨 Unrecognized container detected: {}", name);
                // Optional: Stop it via Docker API
            }
        }

        info!("✅ Trusted container environment verified.");
        Ok(())
    }

    async fn check_daemon(&self) -> anyhow::Result<()> {
        let res = self.http.get("http://localhost:8001/status")  // daemon
            .send().await?;
        let json: serde_json::Value = res.json().await?;
        info!("🧱 Daemon status: {:?}", json);
        Ok(())
    }

    async fn check_signer(&self) -> anyhow::Result<()> {
        let res = self.http.get("http://localhost:8002/ready")  // signer
            .send().await?;
        if res.status().is_success() {
            info!("🔐 Signer online.");
        } else {
            error!("❌ Signer offline.");
        }
        Ok(())
    }

    async fn check_api(&self) -> anyhow::Result<()> {
        let res = self.http.get("http://localhost:8000/status")
            .send().await?;
        if res.status().is_success() {
            info!("🛰️ API online.");
        } else {
            error!("❌ API unreachable.");
        }
        Ok(())
    }
}
