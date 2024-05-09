use self::types::AojProblem;
use crate::domain::problem::Problem;
use anyhow::{Ok, Result};
use std::sync::Arc;
use url::Url;

use super::*;

const AOJ_URL: &str = "https://judgeapi.u-aizu.ac.jp";
const SIZE: usize = 10000; // for now, this size is enogh

pub struct AojAPIClient {
    client: Arc<reqwest::Client>,
}

impl AojAPIClient {
    pub fn new() -> Self {
        return Self {
            client: Arc::new(reqwest::Client::new()),
        };
    }
}

pub trait IAojAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
}

impl IAojAPIClient for AojAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        let url = Url::parse(&format!(
            "{base}/problems?size={size}",
            base = AOJ_URL,
            size = SIZE
        ))
        .unwrap();

        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch aoj problems"));
        }

        let problems = response
            .json::<Vec<AojProblem>>()
            .await?
            .into_iter()
            .map(|p| Problem::try_from(p).unwrap())
            .collect();

        Ok(problems)
    }
}
