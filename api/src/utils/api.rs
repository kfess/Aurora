use anyhow::{Context, Result};
use reqwest::Client;

pub(crate) async fn get_json<T>(url: &str, client: &Client) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
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

pub(crate) async fn get_toml<T>(url: &str, client: &Client) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let text = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to fetch {}", url))?
        .text()
        .await
        .with_context(|| format!("Failed to get text from {}", url))?;

    let toml_data = toml::from_str::<T>(&text).with_context(|| "Failed to deserialize TOML")?;

    Ok(toml_data)
}
