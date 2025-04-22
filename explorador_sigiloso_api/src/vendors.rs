pub async fn fetch_latest_block_height() -> Option<u64> {
    let res = reqwest::get("https://blockstream.info/api/blocks/tip/height")
        .await
        .ok()?;

    let text = res.text().await.ok()?;
    text.parse::<u64>().ok()
}