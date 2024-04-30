use anyhow::{Ok, Result};
use std::env;
use std::sync::Arc;
use url::Url;

use super::types::{GitHubRepoContent, YosupoOnlineJudgeProblem};
use crate::utils::api::build_github_header;

const YOSUPO_ONLINE_JUDGE_GITHUB_URL: &str =
    "https://api.github.com/repos/yosupo06/library-checker-problems/contents/";

pub struct YosupoOnlineJudgeAPIClient {
    client: Arc<reqwest::Client>,
}

impl YosupoOnlineJudgeAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
        }
    }
}

pub trait IYosupoOnlineJudgeAPIClient {
    async fn get_problems(&self) -> Result<Vec<YosupoOnlineJudgeProblem>>;
}

impl IYosupoOnlineJudgeAPIClient for YosupoOnlineJudgeAPIClient {
    async fn get_problems(&self) -> Result<Vec<YosupoOnlineJudgeProblem>> {
        let base_url = Url::parse(YOSUPO_ONLINE_JUDGE_GITHUB_URL)?;

        let mut problems: Vec<YosupoOnlineJudgeProblem> = vec![];

        let categories = vec!["datastructure", "math", "geo", "sample", "graph", "string"];
        let github_access_token = env::var("GITHUB_ACCESS_TOKEN")?;
        for category in categories {
            let url = base_url.join(category)?;
            let headers = build_github_header()?;
            let response = self
                .client
                .get(url)
                .headers(headers)
                .bearer_auth(&github_access_token)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Failed to fetch yosupo online judge problems"
                ));
            }

            let directories = response.json::<Vec<GitHubRepoContent>>().await?;
            directories.iter().for_each(|d| {
                problems.push(YosupoOnlineJudgeProblem {
                    name: d.name.clone(),
                    category: category.to_string(),
                })
            });
        }

        Ok(problems)
    }
}
