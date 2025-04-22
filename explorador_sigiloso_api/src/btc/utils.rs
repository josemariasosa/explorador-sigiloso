pub async fn get_local_heigh() -> Option<u64> {
    let Some(btc) = state.btc.as_ref() else {
        return None;
    };
    
    let best_block_hash = btc.get_best_block_hash().map
    map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get best block: {e}"))
    })?;
}
