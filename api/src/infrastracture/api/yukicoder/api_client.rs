use anyhow::{Ok, Result};
use chrono::DateTime;
use core::time;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use crate::domain::{contest::Contest, problem::Problem, value_object::platform::Platform};

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

    async fn fetch_problem_ids(&self) -> Result<Vec<u64>> {
        let url = format!("{}/problems", YUKICODER_URL);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch yukicoder problems"));
        }

        let problems = response.json::<Vec<YukicoderProblem>>().await?;

        let problem_ids = problems
            .iter()
            .map(|problem| problem.problem_id)
            .collect::<Vec<u64>>();

        Ok(problem_ids)
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

    async fn merge(&self) -> (Vec<Problem>, Vec<Contest>) {
        let (fetched_problem_ids, fetched_contests) = (
            self.fetch_problem_ids().await.unwrap(),
            self.fetch_past_contests().await.unwrap(),
        );

        let (mut problems, mut contests): (Vec<Problem>, Vec<Contest>) = (vec![], vec![]);
        let mut problem_id_map: HashMap<u64, (String, String)> = HashMap::new();
        let mut contest_problems_map: HashMap<String, Vec<Problem>> = HashMap::new();

        for contest in &fetched_contests {
            let mut problem_ids = contest.problem_id_list.clone();
            problem_ids.sort_unstable();
            for (index, problem_id) in problem_ids.iter().enumerate() {
                let letter = ((65u8 + index as u8) as char).to_string();
                problem_id_map.insert(*problem_id, (contest.name.trim().to_string(), letter));
            }
        }

        for problem_id in &fetched_problem_ids {
            let fetched_problem = self.fetch_problem(*problem_id).await.unwrap();

            let default_value = ("Unknown".to_string(), "A".to_string());
            let (contest_name, index) = problem_id_map.get(problem_id).unwrap_or(&default_value);

            let problem = Problem::reconstruct(
                contest_name.to_string(),
                index.to_string(),
                fetched_problem.title.to_string(),
                Platform::Yukicoder,
                Some(fetched_problem.level),
                Option::None,
                fetched_problem
                    .tags
                    .split(",")
                    .map(|s| s.to_string())
                    .collect(),
                format!("https://yukicoder.me/problems/no/{}", fetched_problem.no),
                Some(fetched_problem.statistics.solved),
                Some(fetched_problem.statistics.total),
            );

            problems.push(problem.clone());

            let contest_id = format!("{}_{}", String::from(Platform::Yukicoder), contest_name);
            contest_problems_map
                .entry(contest_id.clone())
                .or_insert(vec![])
                .push(problem.clone());

            thread::sleep(time::Duration::from_millis(1000));
        }

        for contest in fetched_contests {
            let start_timestamp = DateTime::parse_from_rfc3339(&contest.date)
                .unwrap()
                .timestamp() as u64;

            let duration_seconds = DateTime::parse_from_rfc3339(&contest.end_date)
                .unwrap()
                .timestamp() as u64
                - start_timestamp;

            let id = format!("{}_{}", String::from(Platform::Yukicoder), contest.name);

            contests.push(Contest::reconstruct(
                contest.name,
                Platform::Yukicoder,
                "finished".to_string(),
                start_timestamp,
                duration_seconds,
                format!("https://yukicoder.me/contests/{id}", id = contest.id),
                contest_problems_map.get(&id).unwrap().clone(),
            ))
        }

        (problems, contests)
    }
}

pub trait IYukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        let (problems, _) = self.merge().await;

        Ok(problems)
    }
}
