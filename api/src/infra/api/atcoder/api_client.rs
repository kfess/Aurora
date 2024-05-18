use anyhow::{Ok, Result};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    domain::{contest::Contest, problem::Problem, vo::platform},
    utils::api::get_json,
};

use super::{
    classifier::classify_contest,
    external::{AtcoderContest, AtcoderProblem, Estimation},
};

const ATCODER_UNOFFICIAL_API_URL: &'static str = "https://kenkoooo.com/atcoder/resources";

/// AtCoder API Client
/// This struct is used to fetch data from AtCoder API provided from Unofficial AtCoder API (kenkoooo)
pub struct AtcoderAPIClient {
    client: Arc<reqwest::Client>,
    cache: RwLock<Option<(Vec<Problem>, Vec<Contest>)>>,
}

impl AtcoderAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            cache: RwLock::new(None),
        }
    }

    pub async fn fetch_contests(&self) -> Result<Vec<AtcoderContest>> {
        let url = format!("{ATCODER_UNOFFICIAL_API_URL}/contests.json");
        let contests = get_json::<Vec<AtcoderContest>>(&url, &self.client).await?;

        Ok(contests)
    }

    pub async fn fetch_problems(&self) -> Result<Vec<AtcoderProblem>> {
        let url = format!("{ATCODER_UNOFFICIAL_API_URL}/merged-problems.json");
        let problems = get_json::<Vec<AtcoderProblem>>(&url, &self.client).await?;

        Ok(problems)
    }

    pub async fn fetch_estimations(&self) -> Result<HashMap<String, Estimation>> {
        let url = format!("{ATCODER_UNOFFICIAL_API_URL}/problem-models.json");
        let estimations = get_json::<HashMap<String, Estimation>>(&url, &self.client).await?;

        Ok(estimations)
    }

    pub async fn build_problems_contests(&self) -> Result<()> {
        if self.cache.read().unwrap().is_some() {
            return Ok(());
        }

        let estimations = self.fetch_estimations().await?;

        let mut c_to_p_map: HashMap<String, Vec<Problem>> = HashMap::new();
        let raw_problems = self.fetch_problems().await?;
        let mut problems: Vec<Problem> = vec![];
        raw_problems.iter().for_each(|p| {
            let (diff, is_experimental): (Option<f64>, Option<bool>) = match estimations.get(&p.id)
            {
                Some(estimation) => {
                    let clipped_diff = if estimation.difficulty >= 400.0 {
                        estimation.difficulty.round()
                    } else {
                        (400.0 / f64::exp(1.0 - estimation.difficulty / 400.0)).round()
                    };
                    (Some(clipped_diff), Some(estimation.is_experimental))
                }
                None => (None, None),
            };

            let problem = build_problem(p.clone(), diff, is_experimental);
            c_to_p_map
                .entry(p.contest_id.clone())
                .or_insert_with(Vec::new)
                .push(problem.clone());
            problems.push(problem);
        });

        let raw_contests = self.fetch_contests().await?;
        let mut contests: Vec<Contest> = vec![];
        raw_contests.iter().for_each(|c| {
            let contest = build_contest(c.clone(), c_to_p_map.get(&c.id).unwrap().clone());
            contests.push(contest);
        });

        *self.cache.write().unwrap() = Some((problems.clone(), contests.clone()));

        Ok(())
    }
}

pub trait IAtcodedrAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_contests(&self) -> Result<Vec<Contest>>;
}

impl IAtcodedrAPIClient for AtcoderAPIClient {
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

fn build_problem(
    problem: AtcoderProblem,
    difficulty: Option<f64>,
    is_experimental: Option<bool>,
) -> Problem {
    Problem::reconstruct(
        problem.contest_id.to_string(),
        problem.problem_index.to_string(),
        problem.name.to_string(),
        platform::Platform::Atcoder,
        Some(problem.point),
        difficulty,
        is_experimental,
        vec![],
        format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            problem.contest_id, problem.problem_index
        ),
        Some(problem.solver_count),
        None,
    )
}

fn build_contest(contest: AtcoderContest, problems: Vec<Problem>) -> Contest {
    Contest::reconstruct(
        contest.id.to_string(),
        contest.title.to_string(),
        String::from(classify_contest(&contest)),
        platform::Platform::Atcoder,
        "finished".to_string(),
        Some(contest.start_epoch_second),
        Some(contest.duration_second),
        problems,
    )
}