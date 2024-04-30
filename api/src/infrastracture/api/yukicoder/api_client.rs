use anyhow::{Ok, Result};
use std::sync::Arc;

use self::types::{YukicoderContest, YukicoderProblem, YukicoderTag};

use super::*;

const YUKICODER_URL_PREFIX: &str = "https://yukicoder.me/api";
const YUKICODER_API_VER: &str = "v1";

pub struct YukicoderAPIClient {
    client: Arc<reqwest::Client>,
}

impl YukicoderAPIClient {
    pub fn new() -> Self {
        return Self {
            client: Arc::new(reqwest::Client::new()),
        };
    }
}

pub trait IYukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<YukicoderProblem>>;
    async fn get_past_contests(&self) -> Result<Vec<YukicoderContest>>;
    async fn get_future_contests(&self) -> Result<Vec<YukicoderContest>>;
    async fn get_tags(&self) -> Result<Vec<YukicoderTag>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<YukicoderProblem>> {
        let url = format!("{}/{}/problems", YUKICODER_URL_PREFIX, YUKICODER_API_VER);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problems"));
        }

        let problems = response.json::<Vec<YukicoderProblem>>().await?;

        Ok(problems)
    }

    async fn get_past_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!(
            "{}/{}/contest/past",
            YUKICODER_URL_PREFIX, YUKICODER_API_VER
        );
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(contests)
    }

    async fn get_future_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!(
            "{}/{}/contest/future",
            YUKICODER_URL_PREFIX, YUKICODER_API_VER
        );
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let future_contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(future_contests)
    }

    async fn get_tags(&self) -> Result<Vec<YukicoderTag>> {
        let url = format!(
            "{}/{}/statistics/tags",
            YUKICODER_URL_PREFIX, YUKICODER_API_VER
        );
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder tags"));
        }

        let tags = response.json::<Vec<YukicoderTag>>().await?;

        Ok(tags)
    }
}
