mod btc;
mod utils;

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

    // build our application with a single route
    let app = Router::new()
        .route("/btc/balance/{address}", get(btc::handlers::get_balance))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(explorador_url).await.unwrap();
    println!("🚀 Explorador Sigiloso API running at http://{:?}/", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
