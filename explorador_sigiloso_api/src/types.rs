use serde::Serialize;

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