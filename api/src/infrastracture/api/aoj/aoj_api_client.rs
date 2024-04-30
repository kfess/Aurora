use anyhow::{Ok, Result};

use self::types::AojProblem;

use super::*;

const AOJ_URL_PREFIX: &str = "https://judgeapi.u-aizu.ac.jp";
const SIZE: usize = 10000; // for now, this size is enogh

pub struct AojAPIClient;

impl AojAPIClient {
    pub fn new() -> Self {
        return Self {};
    }
}

pub trait IAojAPIClient {
    async fn get_problems(&self) -> Result<Vec<AojProblem>>;
}

impl IAojAPIClient for AojAPIClient {
    async fn get_problems(&self) -> Result<Vec<AojProblem>> {
        let client = reqwest::Client::new();
        let url = format!("{}/problems?size={}", AOJ_URL_PREFIX, SIZE);
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch aoj problems"));
        }

        let problems = response.json::<Vec<AojProblem>>().await?;

        Ok(problems)
    }
}
