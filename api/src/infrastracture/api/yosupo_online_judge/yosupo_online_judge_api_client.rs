use anyhow::{Ok, Result};
use std::env;
use url::Url;

use super::types::{GitHubRepoContent, YosupoOnlineJudgeProblem};
use crate::utils::api::build_github_header;

const YOSUPO_ONLINE_JUDGE_GITHUB_URL: &str =
    "https://api.github.com/repos/yosupo06/library-checker-problems/contents/";

pub struct YosupoOnlineJudgeAPIClient;

impl YosupoOnlineJudgeAPIClient {
    pub fn new() -> Self {
        return Self {};
    }
}

pub trait IYosupoOnlineJudgeAPIClient {
    async fn get_problems(&self) -> Result<Vec<YosupoOnlineJudgeProblem>>;
}

impl IYosupoOnlineJudgeAPIClient for YosupoOnlineJudgeAPIClient {
    async fn get_problems(&self) -> Result<Vec<YosupoOnlineJudgeProblem>> {
        let client = reqwest::Client::new();
        let base_url = Url::parse(YOSUPO_ONLINE_JUDGE_GITHUB_URL)?;

        let mut problems: Vec<YosupoOnlineJudgeProblem> = vec![];

        let categories = vec!["graph"];
        for category in categories {
            let url = base_url.join(category)?;
            let headers = build_github_header()?;

            let response = client
                .get(url)
                .headers(headers)
                .bearer_auth(env::var("GITHUB_ACCESS_TOKEN")?)
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
                    category: format!("{}", category),
                })
            });
        }

        Ok(problems)
    }
}
