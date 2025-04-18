use axum::extract::State;
use axum::{extract::Path, Json};
use bitcoincore_rpc::{Client, RpcApi};
use serde::Serialize;
use std::sync::Arc;

// use std::str::FromStr;
use bitcoin::{Address, Network};
use bitcoin::address::{NetworkUnchecked, NetworkChecked};


#[derive(Serialize)]
pub struct BalanceResponse {
    address: Address<NetworkChecked>,
    balance: u64,
}

#[derive(Serialize)]
struct BlockDelta {
    block_hash: Address<NetworkChecked>,
    height: u64,
    coinbase_reward_sats: u64,
    received_addresses: Vec<Address<NetworkChecked>>,
    spent_addresses: Vec<Address<NetworkChecked>>,
    total_output_sats: u64,
    total_input_sats: u64,
}

pub async fn get_balance(
    Path(address): Path<String>,
    State(rpc): State<Arc<Client>>,
) -> Json<BalanceResponse> {
    // List unspent and filter by address
    let address: Address<NetworkUnchecked> = address.parse().unwrap();
    let address: Address<NetworkChecked> = address.require_network(Network::Bitcoin).unwrap();
    let unspent = rpc.list_unspent(
        None,
        None,
        Some(&[&address]),
        None,
        None
    ).expect("Failed to query unspent outputs");

    let total: u64 = unspent.iter().map(|o| o.amount.to_sat()).sum();

    Json(BalanceResponse {address: address, balance: total})
}
