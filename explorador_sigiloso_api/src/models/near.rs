use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use sqlx_pg_uint::PgU128;

use crate::types::BasisPoint;

#[derive(sqlx::FromRow, Debug)]
pub struct NearValidators {
    id: u64,
    validator_id: String,
    factory_id: String,
    is_online: bool,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct NearValidatorStatus {
    // id: u64,
    pub validator_id: String,
    pub owner_id: String,
    pub total_staked_balance: PgU128,
    pub reward_fee_bp: BasisPoint,
    pub next_reward_fee_bp: BasisPoint,
    pub burn_fee_bp: BasisPoint,
    pub farms: serde_json::Value,
    // snapshot_at: chrono::NaiveDateTime,
}

let u = PgU128::from_str("1234567890123456789012345678901234567890").unwrap();
