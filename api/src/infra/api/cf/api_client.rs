use anyhow::{Ok, Result};
use std::sync::Arc;
use url::Url;

use super::external::{
    CodeforcesContest, CodeforcesContestResponse, CodeforcesProblem, CodeforcesProblemResponse,
};

const CODEFORCES_URL_PREFIX: &str = "https://codeforces.com/api/";

pub struct CodeforcesAPIClient {
    client: Arc<reqwest::Client>,
}

impl CodeforcesAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
        }
    }
}

pub trait ICodeforcesAPICLient {
    async fn get_problems(&self) -> Result<Vec<CodeforcesProblem>>;
    async fn get_contests(&self) -> Result<Vec<CodeforcesContest>>;
}

impl ICodeforcesAPICLient for CodeforcesAPIClient {
    async fn get_problems(&self) -> Result<Vec<CodeforcesProblem>> {
        let url = Url::parse(CODEFORCES_URL_PREFIX)
            .unwrap()
            .join("problemset.problems")
            .unwrap();

        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch codeforces problems"));
        }

        let problems = response
            .json::<CodeforcesProblemResponse>()
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

    async fn get_contests(&self) -> Result<Vec<CodeforcesContest>> {
        let url = Url::parse(CODEFORCES_URL_PREFIX)
            .unwrap()
            .join("contest.list")
            .unwrap();

        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch codeforces contests"));
        }

        let contests = response
            .json::<CodeforcesContestResponse>()
            .await
            .map_err(|e| {
                eprintln!("Failed to parse the codeforces contests JSON: {}", e);
            })
            .unwrap()
            .result
            .unwrap();

        Ok(contests)
    }
}
