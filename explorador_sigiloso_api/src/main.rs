mod btc;
mod utils;
mod types;

use btc::rpc::create_btc_rpc_client;
use types::AppState;
use std::sync::Arc;
use std::env;
use axum::routing::get;
use axum::Router;
// use axum::extract::{Path, State};
// use axum::response::Json;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Read URL from .env file
    let explorador_url = env::var("API_SERVER_BIND").expect("API_SERVER_BIND not set");
    let esplora_url = env::var("ESPLORA_API_URL").expect("ESPLORA_API_URL not set");

    // Build clients
    let btc_rpc = Arc::new(create_btc_rpc_client());
    let esplora_http = reqwest::Client::new();

    // Shared state
    let state = AppState {
        btc: btc_rpc,
        esplora: esplora_http,
        esplora_url,
    };

    // build our application with a multiple routes
    let app = Router::new()
        .route("/btc/balance/{address}", get(btc::handlers::get_balance))
        .route("/btc/block-txs/{block_hash}", get(btc::handlers::block_txs_esplora))
        .route("/btc/last-block-delta", get(btc::handlers::get_last_block_delta))
        .route("/btc/block-delta/{block_hash}", get(btc::handlers::get_block_delta))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(explorador_url.clone()).await.unwrap();
    println!("🚀 Explorador Sigiloso API running at {}", explorador_url);
    axum::serve(listener, app).await.unwrap();
}
