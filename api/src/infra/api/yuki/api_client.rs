use anyhow::{Ok, Result};

use chrono::DateTime;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, sync::RwLock};

use crate::domain::{contest::Contest, problem::Problem, vo::platform::Platform};
use crate::utils::api::{get_json, sleep};
use crate::utils::format::num_to_alphabet;

use crate::infra::api::yuki::types::{
    YukicoderContest, YukicoderProblem, YukicoderProblemWithStatistics, YukicoderTag,
};

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
        let problem = get_json(&url, &self.client).await?;

        Ok(problem)
    }

    async fn fetch_problem_ids(&self) -> Result<Vec<u64>> {
        let url = format!("{}/problems", YUKICODER_URL);
        let problems: Vec<YukicoderProblem> = get_json(&url, &self.client).await?;

        let problem_ids = problems
            .iter()
            .map(|problem| problem.problem_id)
            .collect::<Vec<u64>>();

        Ok(problem_ids)
    }

    async fn fetch_past_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/past", YUKICODER_URL);
        let mut contests: Vec<YukicoderContest> = get_json(&url, &self.client).await?;

        contests.iter_mut().for_each(|contest| {
            contest.name = contest.name.trim().to_string();
        });

        Ok(contests)
    }

    #[allow(dead_code)]
    async fn fetch_future_contests(&self) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/future", YUKICODER_URL);
        let future_contests: Vec<YukicoderContest> = get_json(&url, &self.client).await?;

        Ok(future_contests)
    }

    #[allow(dead_code)]
    async fn fetch_tags(&self) -> Result<Vec<YukicoderTag>> {
        let url = format!("{}/statistics/tags", YUKICODER_URL);
        let tags = get_json(&url, &self.client).await?;

        Ok(tags)
    }

    async fn build_problems_contests(&self) -> Result<()> {
        if self.cache.read().unwrap().is_some() {
            return Ok(());
        }

        let (raw_problem_ids, raw_contests) =
            tokio::try_join!(self.fetch_problem_ids(), self.fetch_past_contests())?;

        let mut p_to_c_map: HashMap<u64, (YukicoderContest, String)> = HashMap::new();
        let mut c_to_p_id_map: HashMap<u64, Vec<Problem>> = HashMap::new();

        for c in raw_contests.iter() {
            for (idx, problem_id) in c.problem_id_list.iter().enumerate() {
                p_to_c_map.insert(*problem_id, (c.clone(), num_to_alphabet(idx)));
            }
        }

        let mut problems: Vec<Problem> = vec![];
        for problem_id in &raw_problem_ids[..3] {
            let raw_problem = self.fetch_problem(*problem_id).await?;
            let (contest, idx) = p_to_c_map.get(problem_id).cloned().unwrap();

            let problem = build_problem(&contest.name, &idx, &raw_problem);
            problems.push(problem.clone());

            c_to_p_id_map
                .entry(contest.id)
                .or_insert_with(Vec::new)
                .push(problem.clone());

            sleep(Duration::from_secs(1)).await;
        }

        let mut contests: Vec<Contest> = vec![];
        for (contest_id, problems) in c_to_p_id_map.iter() {
            let contest = p_to_c_map.get(contest_id).unwrap().0.clone();
            contests.push(build_contest(&contest, problems.clone()));
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
        self.build_problems_contests().await?;
        let cache = self.cache.read().unwrap();
        let (problems, _) = cache.as_ref().unwrap();

        Ok(problems.clone())
    }

    async fn get_contests(&self) -> Result<Vec<Contest>> {
        self.build_problems_contests().await?;
        let cache = self.cache.read().unwrap();
        let (_, contests) = cache.as_ref().unwrap();

        Ok(contests.clone())
    }
}

pub fn build_problem(
    contest_name: &str,
    index: &str,
    problem: &YukicoderProblemWithStatistics,
) -> Problem {
    Problem::reconstruct(
        contest_name.to_string(),
        index.to_string(),
        problem.title.to_string(),
        Platform::Yukicoder,
        Some(problem.level),
        Option::None,
        problem.tags.split(",").map(|s| s.to_string()).collect(),
        format!("https://yukicoder.me/problems/no/{}", problem.no),
        Some(problem.statistics.solved),
        Some(problem.statistics.total),
    )
}

pub fn build_contest(contest: &YukicoderContest, problems: Vec<Problem>) -> Contest {
    let start_timestamp = DateTime::parse_from_rfc3339(&contest.date)
        .unwrap()
        .timestamp() as u64;

    let duration_seconds = DateTime::parse_from_rfc3339(&contest.end_date)
        .unwrap()
        .timestamp() as u64
        - start_timestamp;

    Contest::reconstruct(
        contest.name.to_string(),
        Platform::Yukicoder,
        "finished".to_string(),
        Some(start_timestamp),
        Some(duration_seconds),
        format!("https://yukicoder.me/contests/{}", contest.id),
        problems,
    )
}
