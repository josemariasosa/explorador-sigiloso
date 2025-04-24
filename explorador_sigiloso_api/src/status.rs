use axum::{extract::State, Json};
use crate::types::{AppStatus, AppState};

pub async fn get_app_status(
    State(_state): State<AppState>,
) -> Json<AppStatus> {
    todo!();
    // AppStatus {
    //     bitcoin_node: state.get_bitcoin_node_status(),
    //     // esplora_ok: state.esplora.is_some(),
    //     // external_height: fetch_latest_block_height().await,
    // }
}

// async fn status_handler(state: AppState) -> Json<Status> {
//     let bitcoin_ok = state.rpc.as_ref()
//         .map(|client| client.get_blockchain_info().is_ok())
//         .unwrap_or(false);

//     let esplora_ok = reqwest::get("http://localhost:3002") // or actual esplora API endpoint
//         .await.map(|r| r.status().is_success())
//         .unwrap_or(false);

//     let external = fetch_latest_block_height().await;

//     Json(Status {
//         bitcoin_rpc_ok: bitcoin_ok,
//         esplora_ok,
//         external_height: external,
//     })
// }
