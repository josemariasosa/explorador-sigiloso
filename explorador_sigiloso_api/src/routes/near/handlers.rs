use axum::{extract::{Path, State}, Json};
use bitcoin::address::{NetworkUnchecked, NetworkChecked};
use bitcoin::{Address, BlockHash, Network};
use bitcoincore_rpc::Client as BtcRpcClient;
use bitcoincore_rpc::RpcApi;
use serde::Deserialize;
use serde_json::{from_slice, json};
use crate::app_state::AppState;
use crate::types::{BlockDelta, BalanceResponse};
use crate::utils::expected_block_subsidy;
use reqwest::StatusCode;
use std::{collections::HashMap, vec};
use crate::types::NearValidatorResponse;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::views::QueryRequest;
use near_primitives::types::AccountId;
use methods::query::RpcQueryRequest;
use crate::models::near::NearValidatorStatus;
use sqlx_pg_uint::PgU128;

/// near - handlers
/// Centauri Devs ✨

pub async fn get_and_update_near_validator_stats(
    Path(near_address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<NearValidatorResponse>, (StatusCode, String)> {
    let client = state.near_rpc.as_ref().ok_or(
        (StatusCode::SERVICE_UNAVAILABLE, "NEAR RPC unavailable".to_string())
    )?;

    let validator_id: AccountId = if AccountId::validate(&near_address).is_err() {
        return Err((StatusCode::BAD_REQUEST, "Invalid NEAR account ID".to_string()));
    } else {
        // safe unwrap, since we already validated the address
        near_address.parse::<AccountId>().unwrap()
    };

    let request = RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: validator_id,
            method_name: "get_pool_summary".to_string(),
            args: FunctionArgs::from(
                json!({ })
                .to_string()
                .into_bytes(),
            ),
        },
    };
    let response = client
        .call(request)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("NEAR RPC error: {}", e)))?;

    // 4) Extract and parse the result
    let summary: serde_json::Value = match response.kind {
        QueryResponseKind::CallResult(call) => {
            serde_json::from_slice(&call.result)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON parse error: {}", e)))?
        }
        _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Unexpected RPC response".into())),
    };

    let total_staked_balance: u128 = summary["total_staked_balance"]
        .as_str()
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Invalid total_staked_balance".to_string()))?
        .parse::<u128>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Parse error: {}", e)))?;

    let near_validator_stat = NearValidatorStatus {
        validator_id: near_address.to_string(),
        owner_id: summary["owner"].as_str().unwrap_or("").to_string(),
        total_staked_balance: PgU128::from(total_staked_balance),
        reward_fee_bp: crate::utils::get_basis_point_from(
            summary["reward_fee_bp"]["numerator"].as_u64().unwrap_or(0),
            summary["reward_fee_bp"]["denominator"].as_u64().unwrap_or(0),
        ),
        next_reward_fee_bp: crate::utils::get_basis_point_from(
            summary["next_reward_fee_fraction"]["numerator"].as_u64().unwrap_or(0),
            summary["next_reward_fee_fraction"]["denominator"].as_u64().unwrap_or(0),
        ),
        burn_fee_bp: crate::utils::get_basis_point_from(
            summary["burn_fee_bp"]["numerator"].as_u64().unwrap_or(0),
            summary["burn_fee_bp"]["denominator"].as_u64().unwrap_or(0),
        ),
        farms: vec![],
    };

    // // 5) Persist into your Postgres DB
    // sqlx::query!(
    //     r#"
    //     INSERT INTO near_validator_stats
    //         (validator_id, /* other columns */, created_at)
    //     VALUES
    //         ($1, /* ... */ now())
    //     ON CONFLICT (validator_id) DO UPDATE
    //       SET /* fields = excluded.fields */, updated_at = now()
    //     "#,
    //     near_address, // and other bind params from `summary`
    // )
    // .execute(&state.db_pool as &PgPool)
    // .await
    // .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    // 6) Build your HTTP response
    let resp = NearValidatorResponse {
        pub owner: String,
        pub total_staked_balance: String,
        pub reward_fee_fraction: FeeFraction,
        pub next_reward_fee_fraction: FeeFraction,
        pub burn_fee_fraction: FeeFraction,
        pub farms: Vec<Value>, // change `Value` to a concrete `Farm` struct when you know its fields



        validator_id: near_address,
        total_staked_balance: summary.total_staked_balance,
        // … map other fields …
    };

    Ok(Json(resp))

}