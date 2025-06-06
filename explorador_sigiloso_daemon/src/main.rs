mod state;
mod clients;
mod tasks;

use crate::state::AppState;
use crate::tasks::supervisor::start_supervision;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState::initialize().await;

    // Start background supervisors
    start_supervision(state.clone()).await;

    // Start simple health API
    let app = Router::new()
        .route("/health", get(|| async { "✅ Explorador Sigiloso Daemon is running." }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3500));
    tracing::info!("🚀 Daemon running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
