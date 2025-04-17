use axum::{extract::Path, Json};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Serialize;

// use std::str::FromStr;
use bitcoin::{Address, Network};
use bitcoin::address::{NetworkUnchecked, NetworkChecked};


#[derive(Serialize)]
pub struct BalanceResponse {
    address: String,
    balance: f64,
}

pub async fn get_balance(Path(address): Path<String>) -> Json<BalanceResponse> {
    // Connect to the testnet node
    let rpc = Client::new(
        "http://127.0.0.1:3000", // 3000 for mainnet
        Auth::UserPass("bitcoin".to_string(), "bitcoin123".to_string()),
    ).expect("Failed to connect to Bitcoin RPC");

    // List unspent and filter by address
    let address: Address<NetworkUnchecked> = address.parse().unwrap();
    let address: Address<NetworkChecked> = address.require_network(Network::Testnet).unwrap();
    let unspent = rpc.list_unspent(
        None,
        None,
        Some(&[&address]),
        None,
        None
    ).expect("Failed to query unspent outputs");

    let total: f64 = unspent.iter().map(|o| o.amount.to_btc()).sum();

    Json(BalanceResponse {
        address: address.to_string(),
        balance: total,
    })
}
