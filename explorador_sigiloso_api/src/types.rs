use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use bitcoincore_rpc::Client as BtcRpcClient;
use reqwest::Client as HttpClient;
use crate::models::follower::Follower;
use tokio::sync::mpsc;

pub type BasisPoint = u32;

/// *****************
/// * API Responses *
/// *****************

#[derive(Serialize)]
pub struct NearValidatorResponse {
    pub is_online: bool,
    pub validator_id: String,
    pub factory_id: String,
    pub owner_id: String,
    pub total_staked_balance: u64,
    pub reward_fee_bp: u64,
    pub next_reward_fee_bp: u64,
    pub burn_fee_bp: u64,
    pub farms: Vec<u8>, // TODO: Define the structure of farms
    pub snapshot_at: String,
}

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