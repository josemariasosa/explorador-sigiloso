// pub async fn get_local_heigh() -> Option<u64> {
//     let Some(btc) = state.btc.as_ref() else {
//         return None;
//     };
    
//     let best_block_hash = btc.get_best_block_hash().map
//     map_err(|e| {
//         (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get best block: {e}"))
//     })?;
// }

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use bitcoin::{Address, Network, address::NetworkUnchecked};

pub fn is_bitcoin_mainnet(address_str: &str) -> bool {
    match address_str.parse::<Address<NetworkUnchecked>>() {
        Ok(addr) => addr.require_network(Network::Bitcoin).is_ok(),
        Err(_) => false,
    }
}

// async fn get_latest_block() -> impl IntoResponse {
//     // rpc.get_best_block_hash() & rpc.get_block_count()
//     Json(json!({ "hash": "...", "height": 123456 }))
// }

// async fn get_block(Path(id): Path<String>) -> impl IntoResponse {
//     // decide if `id` is numeric (height) or hex (hash), then rpc.get_block(...)
//     Json(json!({
//         "height": 123456,
//         "hash": id,
//         "tx_count": 2345,
//         "time": 1_700_000_000
//     }))
// }

// async fn get_address_balance(Path(addr): Path<String>) -> impl IntoResponse {
//     // without an indexer you'd have to scan recent blocks—inefficient! 
//     // For now, stub:
//     Json(json!({
//         "address": addr,
//         "total_received": 1_234_5678,
//         "total_sent":  987_6543,
//         "balance":     246_9135
//     }))
// }

// async fn get_address_utxos(Path(addr): Path<String>) -> impl IntoResponse {
//     // stub: return list of UTXOs
//     Json(json!([
//         { "txid": "...", "vout": 0, "value": 50000, "height": 123450 },
//         // …
//     ]))
// }

// async fn get_transaction(Path(txid): Path<String>) -> impl IntoResponse {
//     // rpc.get_raw_transaction(txid, Some(true))
//     Json(json!({ "txid": txid, "inputs": [...], "outputs": [...] }))
// }

