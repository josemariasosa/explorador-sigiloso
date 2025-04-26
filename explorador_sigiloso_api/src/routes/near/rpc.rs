
use std::env;
use std::sync::Arc;
use near_jsonrpc_client::JsonRpcClient;

pub fn try_connect_near() -> Option<Arc<JsonRpcClient>> {
    // 1) RPC call
    let near_rpc = env::var("NEAR_RPC").expect("NEAR_RPC not set");
    let client = JsonRpcClient::connect(near_rpc);

    Some(Arc::new(client))
}


    // let request = methods::query::RpcQueryRequest {
    //     block_reference: methods::query::BlockReference::Finality(methods::query::Finality::Final),
    //     request: methods::query::QueryRequest::ViewFunction {
    //         account_id: account.parse().unwrap(),
    //         method_name: "your_method".to_string(),
    //         args: serde_json::json!({ /* method args */ }).to_string().into_bytes(),
    //     },
    // };

    // let response = client.call(request).await
    //     .map_err(|e| (axum::http::StatusCode::BAD_GATEWAY, e.to_string()))?;