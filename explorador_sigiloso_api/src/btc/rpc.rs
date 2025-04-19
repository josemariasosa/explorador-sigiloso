use bitcoincore_rpc::{Auth, Client};
use std::env;

pub fn create_btc_rpc_client() -> Client {
    let url = env::var("BTC_RPC_URL").expect("BTC_RPC_URL not set");
    let user = env::var("BTC_RPC_USER").expect("BTC_RPC_USER not set");
    let pass = env::var("BTC_RPC_PASS").expect("BTC_RPC_PASS not set");

    Client::new(&url, Auth::UserPass(user, pass)).expect("Failed to connect to Bitcoin RPC")
}
