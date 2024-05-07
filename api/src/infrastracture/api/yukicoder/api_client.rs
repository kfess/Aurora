use anyhow::{Ok, Result};
use chrono::DateTime;
use core::time;
use std::sync::Arc;
use std::thread;
use std::{collections::HashMap, sync::RwLock};

use crate::domain::{contest::Contest, problem::Problem, value_object::platform::Platform};

use self::types::{
    YukicoderContest, YukicoderProblem, YukicoderProblemWithStatistics, YukicoderTag,
};

use super::*;

const YUKICODER_URL: &'static str = "https://yukicoder.me/api/v1";

pub struct YukicoderAPIClient {
    client: Arc<reqwest::Client>,
    cache: RwLock<Option<(Vec<Problem>, Vec<Contest>)>>,
}

impl YukicoderAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            cache: RwLock::new(None),
        }
    }

    async fn fetch_problem(&self, problem_id: u64) -> Result<YukicoderProblemWithStatistics> {
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

        let contests = response
            .json::<Vec<YukicoderContest>>()
            .await?
            .iter()
            .map(|contest| YukicoderContest {
                id: contest.id,
                name: contest.name.trim().to_string(),
                date: contest.date.clone(),
                end_date: contest.end_date.clone(),
                problem_id_list: contest.problem_id_list.clone(),
            })
            .collect();

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

    async fn merge(&self) -> Result<()> {
        if self.cache.read().unwrap().is_some() {
            return Ok(());
        }

        let (fetched_problem_ids, fetched_contests) =
            tokio::try_join!(self.fetch_problem_ids(), self.fetch_past_contests()).unwrap();

        let (mut problems, mut contests): (Vec<Problem>, Vec<Contest>) = (vec![], vec![]);
        let mut problem_id_map: HashMap<u64, (String, String)> = HashMap::new();
        let mut contest_problems_map: HashMap<String, Vec<Problem>> = HashMap::new();

        for contest in fetched_contests.iter() {
            let mut problem_ids = contest.problem_id_list.clone();
            problem_ids.sort_unstable();
            for (index, problem_id) in problem_ids.iter().enumerate() {
                let letter = ((65u8 + index as u8) as char).to_string();
                problem_id_map.insert(*problem_id, (contest.name.to_string(), letter));
            }
        }

        for problem_id in fetched_problem_ids[..100].iter() {
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
                .or_insert_with(Vec::new)
                .push(problem.clone());

            thread::sleep(time::Duration::from_millis(1000));
        }

        for contest in fetched_contests.iter() {
            let start_timestamp = DateTime::parse_from_rfc3339(&contest.date)
                .unwrap()
                .timestamp() as u64;

            let duration_seconds = DateTime::parse_from_rfc3339(&contest.end_date)
                .unwrap()
                .timestamp() as u64
                - start_timestamp;

            let id = format!("{}_{}", String::from(Platform::Yukicoder), contest.name);

            contests.push(Contest::reconstruct(
                contest.name.to_string(),
                Platform::Yukicoder,
                "finished".to_string(),
                start_timestamp,
                duration_seconds,
                format!("https://yukicoder.me/contests/{}", contest.id),
                contest_problems_map.get(&id).cloned().unwrap_or_default(),
            ))
        }

        *self.cache.write().unwrap() = Some((problems.clone(), contests.clone()));

        Ok(())
    }
}

pub trait IYukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_contests(&self) -> Result<Vec<Contest>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        self.merge().await?;
        let cache = self.cache.read().unwrap();
        let (problems, _) = cache.as_ref().unwrap();

        Ok(problems.clone())
    }

    async fn get_contests(&self) -> Result<Vec<Contest>> {
        self.merge().await?;
        let cache = self.cache.read().unwrap();
        let (_, contests) = cache.as_ref().unwrap();

        Ok(contests.clone())
    }
}
