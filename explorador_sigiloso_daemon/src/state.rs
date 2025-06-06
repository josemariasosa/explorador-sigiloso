use std::sync::Arc;
use crate::clients::{eth::EthClient, near::NearClient, btc::BtcClient};

#[derive(Clone)]
pub struct AppState {
    pub eth: Arc<EthClient>,
    pub near: Arc<NearClient>,
    pub btc: Arc<BtcClient>,
}

impl AppState {
    pub async fn initialize() -> Self {
        Self {
            eth: Arc::new(EthClient::new().await),
            near: Arc::new(NearClient::new().await),
            btc: Arc::new(BtcClient::new().await),
        }
    }
}
