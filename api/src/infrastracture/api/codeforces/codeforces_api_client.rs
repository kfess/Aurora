use anyhow::{Ok, Result};
use url::Url;

use super::types::{CodeforcesProblem, CodeforcesProblemAPIResponse, CodeforcesProblemResponse};

const CODEFORCES_URL_PREFIX: &str = "https://codeforces.com/api/";

pub struct CodeforcesAPIClient;

impl CodeforcesAPIClient {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ICodeforcesAPICLient {
    async fn get_problems(&self) -> Result<Vec<CodeforcesProblem>>;
}

impl ICodeforcesAPICLient for CodeforcesAPIClient {
    async fn get_problems(&self) -> Result<Vec<CodeforcesProblem>> {
        let client = reqwest::Client::new();
        let url = Url::parse(CODEFORCES_URL_PREFIX)
            .unwrap()
            .join("problemset.problems")
            .unwrap();

        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch codeforces problems"));
        }

        let problems = response
            .json::<CodeforcesProblemAPIResponse>()
            .await
            .map_err(|e| {
                eprintln!("Failed to parse the codeforces problems JSON: {}", e);
            })
            .unwrap()
            .result
            .unwrap()
            .problems;

        Ok(problems)
    }
}
