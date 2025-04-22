use crate::types::AppState;
use crate::btc::rpc::try_connect_bitcoin;
use std::env;
use crate::types::BitcoinNodeStatus;

pub fn reload() -> AppState {
    // Read URL from .env file
    let esplora_url = env::var("ESPLORA_API_URL").ok();


    // Build clients
    let btc_rpc = try_connect_bitcoin();
    let esplora_http = reqwest::Client::new();
    
    // Shared state
    AppState {
        btc: btc_rpc,
        esplora: Some(esplora_http),
        esplora_url,
    }
}

impl AppState {
    pub fn get_bitcoin_node_status(&self) -> BitcoinNodeStatus {
        BitcoinNodeStatus {
            is_client_ok: self.btc.is_some(),
            is_running: false,
            local_height: self.btc.as_ref().map(|client| client.get_block_count().unwrap_or(0)).unwrap_or(0),
            vendor_height: 0, // Placeholder for vendor height
        }
    }
    
}