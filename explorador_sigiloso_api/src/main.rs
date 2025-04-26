pub mod routes;
mod utils;
mod types;
mod app_state;
mod vendors;
mod status;
mod models;

use std::env;
use axum::routing::{get, post};
use axum::Router;
use routes::btc;
use routes::indexer;
use routes::near;
use app_state::AppState;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Shared state
    let state = AppState::new().await;

    // build our application with a multiple routes
    let app = Router::new()
        .route("/btc/balance/{address}", get(btc::handlers::get_balance))
        // .route("/btc/block-txs/{block_hash}", get(btc::handlers::block_txs_esplora))
        .route("/btc/last-block-delta", get(btc::handlers::get_last_block_delta))
        .route("/btc/block-delta/{block_hash}", get(btc::handlers::get_block_delta))
        .route("/users/{user_id}/refresh", post(indexer::handlers::refresh_user_data))
        .route("/status", get(status::get_app_status))

        // // Friday exploration
        // .route("/btc/latest-block", get(get_latest_block))
        // .route("/btc/block/{id}", get(get_block))
        // .route("/btc/address/{addr}/balance", get(get_address_balance))
        // .route("/btc/address/{addr}/utxos", get(get_address_utxos))
        // .route("/btc/tx/:txid", get(get_transaction))
        // .route("/search", get(search_addresses));
        
        // NEAR routes
        .route("/near/validator/{near_address}", get(near::handlers::get_and_update_near_validator_stats))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let explorador_url = env::var("API_SERVER_BIND").expect("API_SERVER_BIND not set");
    let listener = tokio::net::TcpListener::bind(explorador_url.clone()).await.unwrap();
    println!("🚀 Explorador Sigiloso API running at {}", explorador_url);
    axum::serve(listener, app).await.unwrap();
}
