use anyhow::{Ok, Result};
use std::sync::Arc;

use self::types::{
    YukicoderContest, YukicoderProblem, YukicoderProblemWithStatistics, YukicoderStatistics,
    YukicoderTag,
};

use super::*;

const YUKICODER_URL: &str = "https://yukicoder.me/api/v1";

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
    async fn get_problem(&self, problem_id: u64) -> Result<YukicoderProblemWithStatistics>;
    async fn get_problems(&self) -> Result<Vec<YukicoderProblem>>;
    async fn get_past_contests(&self) -> Result<Vec<YukicoderContest>>;
    async fn get_future_contests(&self) -> Result<Vec<YukicoderContest>>;
    async fn get_tags(&self) -> Result<Vec<YukicoderTag>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_problem(&self, problem_id: u64) -> Result<YukicoderProblemWithStatistics> {
        let url = format!("{}/problems/{}", YUKICODER_URL, problem_id);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problem"));
        }

        let problem = response.json::<YukicoderProblemWithStatistics>().await?;

        Ok(problem)
    }

    async fn get_problems(&self) -> Result<Vec<YukicoderProblem>> {
        let url = format!("{}/problems", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problems"));
        }

        let problems = response.json::<Vec<YukicoderProblem>>().await?;

        Ok(problems)
    }

    async fn get_past_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/past", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(contests)
    }

    async fn get_future_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/future", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let future_contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(future_contests)
    }

    async fn get_tags(&self) -> Result<Vec<YukicoderTag>> {
        let url = format!("{}/statistics/tags", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder tags"));
        }

        let tags = response.json::<Vec<YukicoderTag>>().await?;

        Ok(tags)
    }
}
