use axum::{extract::{Path, State}, Json};
use bitcoin::address::{NetworkUnchecked, NetworkChecked};
use bitcoin::{Address, BlockHash, Network};
use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc::Client as BtcRpcClient;
use crate::types::{BlockDelta, BalanceResponse, AppState};
use crate::utils::expected_block_subsidy;
use std::collections::HashMap;
use reqwest::StatusCode;

pub async fn get_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
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

pub async fn get_last_block_delta(
    State(state): State<AppState>
) -> Result<Json<BlockDelta>, (StatusCode, String)> {
    let Some(btc) = state.btc.as_ref() else {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Bitcoin RPC unavailable".to_string()));
    };

    let best_block_hash = btc.get_best_block_hash().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get best block: {e}"))
    })?;

    Ok(get_block_delta_core(btc, &best_block_hash))
}

pub async fn get_block_delta(
    Path(block_hash): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BlockDelta>, (StatusCode, String)> {
    let Some(btc) = state.btc.as_ref() else {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Bitcoin RPC unavailable".to_string()));
    };

    let block_hash: BlockHash = block_hash.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, "Invalid block hash".to_string())
    })?;

    Ok(get_block_delta_core(btc, &block_hash))
}

fn get_block_delta_core(btc: &BtcRpcClient, block_hash: &BlockHash) -> Json<BlockDelta> {
    println!("🔍 get_block_delta_core called for block hash: {}", block_hash);

    let block = btc.get_block(block_hash).expect("Failed to get block");
    let height = btc.get_block_info(block_hash).unwrap().height;
    let coinbase_tx = block.txdata.first().expect("Block has no transactions");

    // Miners economics ⛏️
    let coinbase_reward_sats = coinbase_tx.output.iter().map(|o| o.value.to_sat()).sum::<u64>();
    let expected_subsidy_sats = expected_block_subsidy(height);
    let total_fees_sats = coinbase_reward_sats.saturating_sub(expected_subsidy_sats);

    let mut total_output = 0;
    let mut total_input = 0;

    let mut received_map: HashMap<String, u64> = HashMap::new();
    let mut spent_map: HashMap<String, u64> = HashMap::new();

    println!(
        "📦 BlockDelta: height={}, outputs={}, inputs={}, rewards={}",
        height, total_output, total_input, coinbase_reward_sats
    );

    for tx in &block.txdata {
        // println!("⛏️ processing tx: {}", tx.txid());
        for o in &tx.output {
            total_output += o.value.to_sat();
            if let Ok(addr) = Address::from_script(&o.script_pubkey, Network::Bitcoin) {
                let addr = addr.to_string();
                *received_map.entry(addr.clone()).or_insert(0) += o.value.to_sat();
            }
        }
        for i in &tx.input {
            if !i.previous_output.is_null() {
                if let Ok(prev_tx) = btc.get_raw_transaction(&i.previous_output.txid, None) {
                    if let Some(prev_out) = prev_tx.output.get(i.previous_output.vout as usize) {
                        total_input += prev_out.value.to_sat();
                        if let Ok(addr) = Address::from_script(&prev_out.script_pubkey, Network::Bitcoin) {
                            let addr = addr.to_string();
                            *spent_map.entry(addr.clone()).or_insert(0) += prev_out.value.to_sat();
                        }
                    }
                }
            }
        }
    }

    // Convert HashMap to Vec for JSON serialization.
    let received_addresses: Vec<(String, u64)> = received_map.into_iter().collect();
    let spent_addresses: Vec<(String, u64)> = spent_map.into_iter().collect();

    Json(BlockDelta {
        block_hash: block.block_hash().to_string(),
        height,
        coinbase_reward_sats,
        expected_subsidy_sats,
        total_fees_sats,
        received_addresses,
        spent_addresses,
        total_output_sats: total_output,
        total_input_sats: total_input,
    })
}




// pub async fn block_txs_esplora(
//     Path(block_hash): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<Value>, (StatusCode, String)> {
//     // call Esplora’s /api/block/:hash/txs
//     let url = format!("{}/api/block/{}/txs", state.esplora_url, block_hash);
//     let resp = state.esplora
//         .get(&url)
//         .send()
//         .await
//         .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

//     let body: Value = resp
//         .json()
//         .await
//         .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

//     Ok(Json(body))
// }
