use anyhow::{Ok, Result};

use chrono::{DateTime, Duration, Local};
use std::sync::Arc;
use std::{collections::HashMap, sync::RwLock};
use tokio::time::{sleep, Duration as TokioDuration};

use crate::domain::{contest::Contest, problem::Problem, vo::platform::Platform};
use crate::utils::api::get_json;
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
        let problem = get_json::<YukicoderProblemWithStatistics>(&url, &self.client).await?;

        Ok(problem)
    }

    async fn fetch_problem_ids(&self, is_recent: bool) -> Result<Vec<u64>> {
        let url = format!("{}/problems", YUKICODER_URL);
        let mut problems = get_json::<Vec<YukicoderProblem>>(&url, &self.client).await?;

        if is_recent {
            let a_week_ago = Local::now() - Duration::days(7);

            problems = problems
                .into_iter()
                .filter(|p| match DateTime::parse_from_rfc3339(&p.date) {
                    std::result::Result::Ok(parsed_date) => {
                        parsed_date.timestamp() > a_week_ago.timestamp()
                    }
                    Err(_) => false,
                })
                .collect();
        }

        let problem_ids = problems
            .iter()
            .map(|problem| problem.problem_id)
            .collect::<Vec<u64>>();

        Ok(problem_ids)
    }

    async fn fetch_past_contests(&self, is_recent: bool) -> Result<Vec<YukicoderContest>> {
        let url = format!("{}/contest/past", YUKICODER_URL);
        let mut contests: Vec<YukicoderContest> = get_json(&url, &self.client).await?;

        if is_recent {
            let a_week_ago = Local::now() - Duration::days(7);

            contests = contests
                .into_iter()
                .filter(|c| match DateTime::parse_from_rfc3339(&c.date) {
                    std::result::Result::Ok(parsed_date) => {
                        parsed_date.timestamp() > a_week_ago.timestamp()
                    }
                    Err(_) => false,
                })
                .collect();
        }

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

    async fn build_problems_contests(&self, is_recent: bool) -> Result<()> {
        if self.cache.read().unwrap().is_some() {
            return Ok(());
        }

        let (raw_problem_ids, raw_contests) = tokio::try_join!(
            self.fetch_problem_ids(is_recent),
            self.fetch_past_contests(is_recent)
        )?;

        let mut p_to_c_map: HashMap<u64, (YukicoderContest, String)> = HashMap::new();
        let mut c_to_p_id_map: HashMap<u64, Vec<Problem>> = HashMap::new();
        let mut c_to_c_map: HashMap<u64, YukicoderContest> = HashMap::new();

        for c in &raw_contests {
            for (idx, &problem_id) in c.problem_id_list.iter().enumerate() {
                p_to_c_map.insert(problem_id, (c.clone(), num_to_alphabet(idx)));
                c_to_c_map.insert(c.id, c.clone());
            }
        }

        let mut problems: Vec<Problem> = vec![];
        for &problem_id in &raw_problem_ids {
            let raw_problem = self.fetch_problem(problem_id).await?;
            let (contest, idx) = p_to_c_map.get(&problem_id).cloned().unwrap();

            let problem = build_problem(&contest.id, &idx, &raw_problem);
            problems.push(problem.clone());

            c_to_p_id_map
                .entry(contest.id)
                .or_insert_with(Vec::new)
                .push(problem.clone());

            sleep(TokioDuration::from_secs(1)).await;
        }

        let contests: Vec<Contest> = c_to_p_id_map
            .iter()
            .map(|(&id, problems)| build_contest(&c_to_c_map[&id], problems.clone()))
            .collect();

        *self.cache.write().unwrap() = Some((problems.clone(), contests.clone()));

        Ok(())
    }
}

pub trait IYukicoderAPIClient {
    async fn get_problems(&self, is_recent: bool) -> Result<Vec<Problem>>;
    async fn get_contests(&self, is_recent: bool) -> Result<Vec<Contest>>;
}

impl IYukicoderAPIClient for YukicoderAPIClient {
    async fn get_problems(&self, is_recent: bool) -> Result<Vec<Problem>> {
        self.build_problems_contests(is_recent).await?;
        let cache = self.cache.read().unwrap();
        let (problems, _) = cache.as_ref().unwrap();

        Ok(problems.clone())
    }

    async fn get_contests(&self, is_recent: bool) -> Result<Vec<Contest>> {
        self.build_problems_contests(is_recent).await?;
        let cache = self.cache.read().unwrap();
        let (_, contests) = cache.as_ref().unwrap();

        Ok(contests.clone())
    }
}

fn build_problem(
    contest_id: &u64,
    index: &str,
    problem: &YukicoderProblemWithStatistics,
) -> Problem {
    Problem::reconstruct(
        contest_id.to_string(),
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

fn build_contest(contest: &YukicoderContest, problems: Vec<Problem>) -> Contest {
    let start_timestamp = DateTime::parse_from_rfc3339(&contest.date)
        .unwrap()
        .timestamp() as u64;

    let duration_seconds = DateTime::parse_from_rfc3339(&contest.end_date)
        .unwrap()
        .timestamp() as u64
        - start_timestamp;

    Contest::reconstruct(
        contest.id.to_string(),
        contest.name.to_string(),
        Platform::Yukicoder,
        "finished".to_string(),
        Some(start_timestamp),
        Some(duration_seconds),
        problems,
    )
}
