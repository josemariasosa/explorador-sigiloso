use crate::state::AppState;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

pub async fn start_supervision(state: AppState) {
    let eth = state.eth.clone();
    let near = state.near.clone();
    let btc = state.btc.clone();

    tokio::spawn(async move {
        loop {
            if !eth.is_healthy().await {
                warn!("⚠️ ETH node unhealthy, consider restarting...");
            }
            sleep(Duration::from_secs(30)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            if !near.is_healthy().await {
                warn!("⚠️ NEAR node unhealthy, consider restarting...");
            }
            sleep(Duration::from_secs(30)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            if !btc.is_healthy().await {
                warn!("⚠️ BTC node unhealthy, consider restarting...");
            }
            sleep(Duration::from_secs(30)).await;
        }
    });

    info!("👀 Supervisor tasks launched");
}
