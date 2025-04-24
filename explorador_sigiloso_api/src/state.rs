use crate::btc::rpc::try_connect_bitcoin;
use crate::indexer::try_connect_postgres;
use crate::models::follower::Follower;
use crate::types::AppState;
use crate::types::BitcoinNodeStatus;
use std::env;
use tokio::sync::mpsc;


impl AppState {
    /// Creates a brand-new state, connecting to all services.
    pub async fn new() -> Self {
        // Read URL from .env file
        let esplora_url = env::var("ESPLORA_API_URL").ok();

        // Build clients
        let btc_rpc = try_connect_bitcoin();
        let esplora_http = reqwest::Client::new();
        let postgres_conn = try_connect_postgres().await;

        // your job queue channel
        let (job_tx, mut job_rx) = mpsc::channel::<Follower>(100);

        // // spawn your background worker
        // tokio::spawn(async move {
        //     while let Some(follower) = job_rx.recv().await {
        //         // process follower…
        //     }
        // });

        AppState {
            db: postgres_conn,
            btc: btc_rpc,
            esplora: Some(esplora_http),
            esplora_url,
            job_tx,
        }
    }

    /// Replaces the current state by reconnecting everything.
    pub async fn reload(&mut self) {
        *self = AppState::new().await;
    }
}



impl AppState {
    pub fn get_bitcoin_node_status(&self) -> BitcoinNodeStatus {
        todo!();
        // BitcoinNodeStatus {
        //     is_client_ok: self.btc.is_some(),
        //     is_running: false,
        //     local_height: self.btc.as_ref().map(|client| client.get_block_count().unwrap_or(0)).unwrap_or(0),
        //     vendor_height: 0, // Placeholder for vendor height
        // }
    }
    
}