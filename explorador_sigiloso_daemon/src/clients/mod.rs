// src/clients/mod.rs

pub mod btc;
// pub mod eth;
// pub mod near;

pub use btc::BtcClient;

use async_trait::async_trait;

#[async_trait]
pub trait NodeClient: Send + Sync {
    fn name(&self) -> &'static str;
    async fn is_healthy(&self) -> bool;
    async fn fetch_log_tail(&self, lines: usize) -> Vec<String>;
    async fn restart(&self) -> Result<(), String>;
}
// This way, other modules (like your AppState or CLI logic) can simply use:

// rust
// Copy
// Edit
// use crate::clients::{NodeClient, BtcClient};