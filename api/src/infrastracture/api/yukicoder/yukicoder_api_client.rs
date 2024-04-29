use anyhow::{Ok, Result};

use self::types::{YukicoderContest, YukicoderProblem};

use super::*;

const YUKICODER_URL_PREFIX: &str = "https://yukicoder.me/api";
const YUKICODER_API_VER: &str = "v1";

pub struct YukicoderAPIClient;

impl YukicoderAPIClient {
    pub fn new() -> Self {
        return Self {};
    }
}

pub trait IYukicoderAPIClient {
    async fn get_all_problems(&self) -> Result<Vec<YukicoderProblem>>;
    async fn get_past_contests(&self) -> Result<Vec<YukicoderContest>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_all_problems(&self) -> Result<Vec<YukicoderProblem>> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}/problems", YUKICODER_URL_PREFIX, YUKICODER_API_VER);
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problems"));
        }

        let problems = response.json::<Vec<YukicoderProblem>>().await?;

        Ok(problems)
    }

    async fn get_past_contests(&self) -> Result<Vec<YukicoderContest>> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/{}/contest/past",
            YUKICODER_URL_PREFIX, YUKICODER_API_VER
        );
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(contests)
    }
}
