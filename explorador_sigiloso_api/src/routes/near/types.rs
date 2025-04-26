use serde::{Deserialize, Serialize};
use serde_json::Value;

// / ****************************
// / * NEAR Contracts Responses *
// / ****************************

// #[derive(Debug, Serialize, Deserialize)]
// pub struct FeeFraction {
//     pub numerator: u64,
//     pub denominator: u64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// /// Response: `nodosigiloso.pool.near.get_pool_summary()`
// pub struct NearPoolSummaryResponse {
//     pub owner: String,
//     pub total_staked_balance: String,
//     pub reward_fee_fraction: FeeFraction,
//     pub next_reward_fee_fraction: FeeFraction,
//     pub burn_fee_fraction: FeeFraction,
//     pub farms: Vec<Value>, // change `Value` to a concrete `Farm` struct when you know its fields
// }