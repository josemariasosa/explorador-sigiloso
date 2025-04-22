use serde::Serialize;
use std::sync::Arc;
use bitcoincore_rpc::Client as BtcRpcClient;
use reqwest::Client as HttpClient;

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: u64,
}

#[derive(Serialize)]
pub struct BlockDelta {
    pub block_hash: String,
    pub height: usize,
    pub coinbase_reward_sats: u64,
    pub expected_subsidy_sats: u64,
    pub total_fees_sats: u64,
    pub received_addresses: Vec<(String, u64)>,
    pub spent_addresses: Vec<(String, u64)>,
    pub total_output_sats: u64,
    pub total_input_sats: u64,
}

#[derive(Clone)]
pub struct AppState {
    /// Bitcoin‐Core RPC client
    pub btc: Option<Arc<BtcRpcClient>>,
    /// an HTTP client for Esplora
    pub esplora: Option<HttpClient>,
    /// base URL for the Esplora‐indexer service
    pub esplora_url: Option<String>,
}

#[derive(Serialize)]
pub struct BitcoinNodeStatus {
    pub is_client_ok: bool,
    pub is_running: bool,
    /// @dev height of the local Bitcoin node
    pub local_height: u64,
    /// @dev height of the Bitcoin network
    pub vendor_height: u64,
}

#[derive(Serialize)]
pub struct AppStatus {
    pub bitcoin_node: BitcoinNodeStatus,
    pub esplora_ok: bool,
    pub external_height: Option<u64>,
}