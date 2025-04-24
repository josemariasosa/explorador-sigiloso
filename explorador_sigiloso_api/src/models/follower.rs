use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(sqlx::FromRow, Debug)]
pub struct Follower {
    id: u64,
    user_id: u64,
    label: String,
    kind: String,
    target: serde_json::Value,
    public: bool,
    created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct FollowerSnapshot {
    pub id: u64,
    pub follower_id: u64,
    pub data: serde_json::Value,
    pub taken_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]  // optional, for clarity
#[sqlx(rename_all = "lowercase")]  // maps to lowercase strings in DB
pub enum NearValidatorPoolVersion {
    V1_POOLV1_NEAR,
    UPGRADEABLE_POOL_NEAR,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]  // optional, for clarity
#[sqlx(rename_all = "lowercase")]  // maps to lowercase strings in DB
pub enum LocalFiatCurrency {
    USD,
    MXN,
}

pub struct NearValidatorData {
    pub id: u32,
    /// Validator Contract Address
    /// e.g. nodosigiloso.pool.near
    pub pool_id: u32,
    /// Validator Pool Version
    pub pool_version: NearValidatorPoolVersion,
    /// Near address like nodosigiloso.near
    pub owner_id: String,
    // MXN or USD
    pub user_local_currency: LocalFiatCurrency,
    /// Commission Rate (%) charged by the validator
    /// 100% -> 10000 basis points
    pub rewards_fee: u32,
    /// Total NEAR **All NEAR amounts are in yocto NEAR**
    /// for local currency **All amounts have ONLY 2 decimals 77700 -> 777MXN**
    pub total_staked_near: u128,
    /// Total Staked in local currency
    pub total_staked_local: u32,
    /// Your Own Stake (NEAR)
    pub own_stake_near: u128,
    pub own_stake_local: u128,
    /// Estimation of the Annual Reward Rate (APR) in basis points
    pub apr: u32,
    /// Uptime Performance (%)
    pub current_uptime_performance: u32,
    /// Uptime Performance Adjusted (%)
    pub current_uptime_adjusted_performance: u32,
    /// Estimated Monthly NEAR Rewards
    pub monthly_rewards_near: u128,
    /// Estimated Your Own Stake Rewards (NEAR)
    pub own_stake_rewards_near: u128,
    /// Estimated Delegators’ Rewards (NEAR)
    pub delegators_rewards_near: u128,
    /// Estimated Validator Commission (NEAR)
    pub validator_commission_near: u128,
    /// Estimated Monthly Validator Earnings (NEAR)
    pub monthly_validator_earnings_near: u128,
    /// Estimated Monthly Validator Earnings (MXN)
    pub monthly_validator_earnings_local: u128,
    /// Monthly Validator Actual NEAR
    pub monthly_validator_actual_near: u128,
    /// Expenses MXN
    pub expenses_local: u128,
    /// Expenses NEAR
    pub expenses_near: u128,
    /// Total Expenses MXN
    pub total_expenses_local: u128,
    /// Profits MXN
    pub exercise_profits_local: f64,
    /// Total Including crypto-asset MXN
    pub total_including_crypto_local: f64,
    /// Asset detail hardware-assets MXN
    pub hardware_assets_value_local: f64,
    /// Asset detail crypto-asset MXN
    pub crypto_assets_value_local: f64,
    /// Total including all assets MXN
    pub exercise_total_including_all_assets_local: f64,
}
