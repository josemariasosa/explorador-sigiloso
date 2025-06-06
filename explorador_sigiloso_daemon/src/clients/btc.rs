// src/clients/btc.rs
use async_trait::async_trait;
use bollard::Docker;
use bollard::container::{InspectContainerOptions, ListContainersOptions};
use tracing::info;
use crate::clients::NodeClient;

pub struct BtcClient {
    docker: Docker,
    container_name: String,
}

impl BtcClient {
    pub async fn new() -> Self {
        let docker = Docker::connect_with_local_defaults().expect("Failed to connect to Docker daemon");
        Self {
            docker,
            container_name: "bitcoin-mainnet".to_string(),
        }
    }
}

#[async_trait]
impl NodeClient for BtcClient {
    fn name(&self) -> &'static str {
        "bitcoin"
    }

    

    async fn is_healthy(&self) -> bool {
        let filters = hashmap!("name" => vec![self.container_name.clone()]);
        let containers = self.docker.list_containers(Some(ListContainersOptions::<String> {
            all: true,
            filters,
            ..Default::default()
        })).await;

        match containers {
            Ok(list) => !list.is_empty() && list[0].state == "running",
            Err(_) => false,
        }
    }

    async fn fetch_log_tail(&self, _lines: usize) -> Vec<String> {
        // Optional: Implement log reading with docker logs
        vec![]
    }

    async fn restart(&self) -> Result<(), String> {
        self.docker.restart_container(&self.container_name, None)
            .await
            .map_err(|e| format!("Restart failed: {}", e))
    }
} 


//     pub async fn run_bitcoin_cli(&self, args: Vec<&str>) -> Result<String, String> {
//         let create = CreateExecOptions {
//             attach_stdout: Some(true),
//             attach_stderr: Some(true),
//             cmd: Some(["bitcoin-cli"].iter().chain(args.iter()).map(|s| s.to_string()).collect()),
//             ..Default::default()
//         };

//         let exec = self.docker.create_exec(&self.container_name, create)
//             .await.map_err(|e| format!("CreateExec failed: {}", e))?;

//         let output = self.docker.start_exec(&exec.id, None::<StartExecOptions>)
//             .await.map_err(|e| format!("StartExec failed: {}", e))?;

//         let mut result = String::new();
//         match output {
//             bollard::exec::StartExecResults::Attached { mut output, .. } => {
//                 use futures_util::stream::TryStreamExt;
//                 while let Some(Ok(msg)) = output.try_next().await {
//                     if let bollard::container::LogOutput::StdOut { message } = msg {
//                         result.push_str(&String::from_utf8_lossy(&message));
//                     }
//                 }
//             }
//             _ => {}
//         }

//         Ok(result.trim().to_string())
//     }
// }