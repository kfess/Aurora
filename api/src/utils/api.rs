use anyhow::{Context, Result};
use reqwest::Client;
use tokio::time;

pub(crate) async fn get_json<T: serde::de::DeserializeOwned>(
    url: &str,
    client: &Client,
) -> Result<T> {
    client
        .get(url)
        .header("accept", "application/json")
        .send()
        .await
        .with_context(|| format!("Failed to fetch {}", url))
        .unwrap()
        .json::<T>()
        .await
        .with_context(|| format!("Failed to parse json from {}", url))
}

pub(crate) async fn sleep(duration: std::time::Duration) {
    time::sleep(duration).await;
}
