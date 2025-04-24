use bitcoincore_rpc::{Auth, Client};
use std::env;
use std::sync::Arc;

pub fn try_connect_bitcoin() -> Option<Arc<Client>> {
    let url = env::var("BTC_RPC_URL").expect("BTC_RPC_URL not set");
    let user = env::var("BTC_RPC_USER").expect("BTC_RPC_USER not set");
    let pass = env::var("BTC_RPC_PASS").expect("BTC_RPC_PASS not set");

    match Client::new(&url, Auth::UserPass(user, pass)) {
        Ok(client) => Some(Arc::new(client)),
        Err(_) => None,
    }
}
