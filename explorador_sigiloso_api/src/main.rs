mod btc;
mod utils;
mod types;
mod state;
mod vendors;
mod status;

use std::env;
use axum::routing::get;
use axum::Router;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Read URL from .env file
    let explorador_url = env::var("API_SERVER_BIND").expect("API_SERVER_BIND not set");

    // Shared state
    let state = state::reload();

    // build our application with a multiple routes
    let app = Router::new()
        .route("/btc/balance/{address}", get(btc::handlers::get_balance))
        // .route("/btc/block-txs/{block_hash}", get(btc::handlers::block_txs_esplora))
        .route("/btc/last-block-delta", get(btc::handlers::get_last_block_delta))
        .route("/btc/block-delta/{block_hash}", get(btc::handlers::get_block_delta))
        .route("/status", get(status::get_app_status))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(explorador_url.clone()).await.unwrap();
    println!("🚀 Explorador Sigiloso API running at {}", explorador_url);
    axum::serve(listener, app).await.unwrap();
}
