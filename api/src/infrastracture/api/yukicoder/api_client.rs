use anyhow::{Ok, Result};
use core::time;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use crate::domain::{problem::Problem, value_object::platform::Platform};

use self::types::{
    YukicoderContest, YukicoderProblem, YukicoderProblemWithStatistics, YukicoderTag,
};

use super::*;

const YUKICODER_URL: &'static str = "https://yukicoder.me/api/v1";

pub struct YukicoderAPIClient {
    client: Arc<reqwest::Client>,
}

impl YukicoderAPIClient {
    pub fn new() -> Self {
        return Self {
            client: Arc::new(reqwest::Client::new()),
        };
    }

    async fn fetch_problem(&self, problem_id: u64) -> Result<YukicoderProblemWithStatistics> {
        println!("Fetching yukicoder problem: {}", problem_id);
        let url = format!("{}/problems/{}", YUKICODER_URL, problem_id);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problem"));
        }

        let problem = response.json::<YukicoderProblemWithStatistics>().await?;

        Ok(problem)
    }

    async fn fetch_problems(&self) -> Result<Vec<YukicoderProblem>> {
        let url = format!("{}/problems", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problems"));
        }

        let problems = response.json::<Vec<YukicoderProblem>>().await?;

        Ok(problems)
    }

    async fn fetch_past_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/past", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(contests)
    }

    #[allow(dead_code)]
    async fn fetch_future_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/future", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder contests"));
        }

        let future_contests = response.json::<Vec<YukicoderContest>>().await?;

        Ok(future_contests)
    }

    #[allow(dead_code)]
    async fn fetch_tags(&self) -> Result<Vec<YukicoderTag>> {
        let url = format!("{}/statistics/tags", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder tags"));
        }

        let tags = response.json::<Vec<YukicoderTag>>().await?;

        Ok(tags)
    }

    async fn merge(&self) -> Vec<Problem> {
        let mut problem_id_map: HashMap<u64, (String, String)> = HashMap::new();

        let contests = self.fetch_past_contests().await.unwrap();
        for contest in contests {
            let mut problem_ids = contest.problem_id_list;
            problem_ids.sort();
            for (index, problem_id) in problem_ids.iter().enumerate() {
                let letter = ((65u8 + index as u8) as char).to_string();
                problem_id_map.insert(*problem_id, (contest.name.to_string(), letter));
            }
        }

        let problem_ids = self
            .fetch_problems()
            .await
            .unwrap()
            .iter()
            .map(|problem| problem.problem_id)
            .collect::<Vec<u64>>();

        let mut problems: Vec<Problem> = vec![];

        for problem_id in problem_ids[..100].iter() {
            let problem = self.fetch_problem(*problem_id).await.unwrap();

            thread::sleep(time::Duration::from_millis(1000));

            let default_value = ("Unknown".to_string(), "A".to_string());
            let (contest_name, index) = problem_id_map.get(problem_id).unwrap_or(&default_value);
            let success_rate = match problem.statistics.total {
                0 => None,
                _ => {
                    Some(problem.statistics.solved as f64 / problem.statistics.total as f64 * 100.0)
                }
            };
            problems.push(Problem::reconstruct(
                format!(
                    "{platform}_{name}_{index}",
                    platform = String::from(Platform::Yukicoder),
                    name = contest_name,
                    index = index
                ),
                format!(
                    "{platform}_{name}",
                    platform = String::from(Platform::Yukicoder),
                    name = contest_name
                ),
                index.to_string(),
                problem.title.to_string(),
                format!(
                    "{index}. {name}",
                    index = index,
                    name = problem.title.to_string()
                ),
                Platform::Yukicoder,
                Some(problem.level),
                Option::None,
                problem.tags.split(",").map(|s| s.to_string()).collect(),
                Some(problem.statistics.solved),
                Some(problem.statistics.total),
                success_rate,
            ))
        }

        problems
    }
}

pub trait IYukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        let problems = self.merge().await;
        Ok(problems)
    }
}
