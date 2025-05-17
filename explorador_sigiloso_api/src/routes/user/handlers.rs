use axum::{extract::{Path, State}, Json};
use bitcoin::address::{NetworkUnchecked, NetworkChecked};
use bitcoin::{Address, BlockHash, Network};
use bitcoincore_rpc::Client as BtcRpcClient;
use bitcoincore_rpc::RpcApi;
use crate::app_state::AppState;
use crate::types::{BlockDelta, BalanceResponse};
use crate::utils::expected_block_subsidy;
use reqwest::StatusCode;
use std::collections::HashMap;

use crate::routes::btc::utils::is_bitcoin_mainnet;

struct AddressProfile {
    identifier: String,
    is_bitcoin_mainnet: bool,
    is_near_mainnet: bool,
    is_eth_mainnet: bool,
}

async fn get_address_profile(address: String) -> AddressProfile {
    // Placeholder for actual implementation

    let address = address.to_lowercase();

    let is_near_mainnet = match near_sdk::AccountId::validate(&address) {
        Ok(_) => true,
        Err(_) => false,
    };

    AddressProfile {
        identifier: address.to_string(),
        is_bitcoin_mainnet: is_bitcoin_mainnet(&address),
        is_near_mainnet,
        is_eth_mainnet: false,
    }
}

pub async fn get_user_home(
    Path(identifier): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {

    let profile = get_address_profile(identifier).await;

    let Some(btc_client) = &state.btc else {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Bitcoin RPC unavailable".to_string()));
    };

    let address: Address<NetworkUnchecked> = address.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, "Invalid Bitcoin address".to_string())
    })?;
    let address: Address<NetworkChecked> = address.require_network(Network::Bitcoin).map_err(|_| {
        (StatusCode::BAD_REQUEST, "Invalid network for address".to_string())
    })?;

    let unspent = btc_client.list_unspent(
        None,
        None,
        Some(&[&address]),
        None,
        None
    ).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("BTC RPC error: {}", e))
    })?;

    let balance: u64 = unspent.iter().map(|o| o.amount.to_sat()).sum();

    Ok(Json(BalanceResponse {
        address: address.to_string(),
        balance,
    }))
}