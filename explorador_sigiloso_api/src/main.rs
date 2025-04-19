mod btc;
mod utils;
mod types;

use btc::rpc::create_rpc_client;
use std::sync::Arc;

use std::env;


use axum::{
    routing::get,
    Router,
};
// use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let explorador_url = env::var("API_SERVER_BIND").expect("API_SERVER_BIND not set");
    let rpc_client = Arc::new(create_rpc_client());
    let app_state = rpc_client.clone();

    // build our application with a multiple routes
    let app = Router::new()
        .route("/btc/balance/{address}", get(btc::handlers::get_balance))
        .route("/btc/last-block-delta", get(btc::handlers::get_last_block_delta))
        .route("/btc/block-delta/{block_hash}", get(btc::handlers::get_block_delta))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(explorador_url.clone()).await.unwrap();
    println!("🚀 Explorador Sigiloso API running at {}", explorador_url);
    axum::serve(listener, app).await.unwrap();
}
