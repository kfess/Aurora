use anyhow::{Ok, Result};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    domain::{contest::Contest, problem::Problem, vo::platform::Platform},
    utils::api::get_json,
};

use super::{
    classifier::classify_contest,
    external::{
        CodeforcesContest, CodeforcesContestResponse, CodeforcesProblem, CodeforcesProblemResponse,
        CodeforcesProblemStat,
    },
};

const CODEFORCES_URL_PREFIX: &'static str = "https://codeforces.com/api";

pub struct CodeforcesAPIClient {
    client: Arc<reqwest::Client>,
    cache: RwLock<Option<(Vec<Problem>, Vec<Contest>)>>,
}

impl CodeforcesAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            cache: RwLock::new(None),
        }
    }

    async fn fetch_problems(&self) -> Result<(Vec<CodeforcesProblem>, Vec<CodeforcesProblemStat>)> {
        let url = format!("{CODEFORCES_URL_PREFIX}/problemset.problems");
        let result = get_json::<CodeforcesProblemResponse>(&url, &self.client).await?;
        let problems_with_stats = result.result.unwrap();

        Ok((
            problems_with_stats.problems,
            problems_with_stats.problem_statistics,
        ))
    }

    async fn fetch_past_contests(&self) -> Result<Vec<CodeforcesContest>> {
        let url = format!("{CODEFORCES_URL_PREFIX}/contest.list");
        let result = get_json::<CodeforcesContestResponse>(&url, &self.client).await?;
        let contests = result
            .result
            .unwrap()
            .into_iter()
            .filter(|c| c.phase == "FINISHED")
            .collect::<Vec<CodeforcesContest>>();

        Ok(contests)
    }

    #[allow(dead_code)]
    async fn fetch_future_contests(&self) -> Result<Vec<CodeforcesContest>> {
        let url = format!("{CODEFORCES_URL_PREFIX}/contest.list");
        let result = get_json::<CodeforcesContestResponse>(&url, &self.client).await?;
        let contests = result
            .result
            .unwrap()
            .into_iter()
            .filter(|c| c.phase == "BEFORE")
            .collect::<Vec<CodeforcesContest>>();

        Ok(contests)
    }

    async fn build_problems_contests(&self) -> Result<()> {
        if self.cache.read().unwrap().is_some() {
            return Ok(());
        }

        let (raw_problems, raw_stats) = self.fetch_problems().await?;
        let raw_contests = self.fetch_past_contests().await?;

        let id_to_solved_count: HashMap<u64, u64> = raw_stats
            .iter()
            .map(|s| (s.contest_id, s.solved_count))
            .collect();

        let mut c_to_p_map: HashMap<u64, Vec<Problem>> = HashMap::new();

        let mut problems: Vec<Problem> = vec![];
        raw_problems.iter().for_each(|p| {
            let problem = build_problem(p.clone(), id_to_solved_count.clone());
            c_to_p_map
                .entry(p.contest_id)
                .or_insert_with(Vec::new)
                .push(problem.clone());
            problems.push(problem);
        });

        let mut contests: Vec<Contest> = vec![];
        raw_contests.iter().for_each(|c| {
            let ps: Vec<Problem> = c_to_p_map.get(&c.id).unwrap().clone();
            contests.push(build_contest(c.clone(), ps));
        });

        *self.cache.write().unwrap() = Some((problems, contests));

        Ok(())
    }
}

pub trait ICodeforcesAPICLient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_contests(&self) -> Result<Vec<Contest>>;
}

impl ICodeforcesAPICLient for CodeforcesAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        self.fetch_problems().await?;
        let cache = self.cache.read().unwrap();
        let (problems, _) = cache.as_ref().unwrap();

        Ok(problems.clone())
    }

    async fn get_contests(&self) -> Result<Vec<Contest>> {
        self.fetch_past_contests().await?;
        let cache = self.cache.read().unwrap();
        let (_, contests) = cache.as_ref().unwrap();

        Ok(contests.clone())
    }
}

fn build_problem(problem: CodeforcesProblem, id_to_solved_count: HashMap<u64, u64>) -> Problem {
    Problem::reconstruct(
        problem.contest_id.to_string(),
        problem.index.clone(),
        problem.name.clone(),
        Platform::Codeforces,
        problem.points,
        problem.rating,
        Some(false),
        problem.tags.clone(),
        format!(
            "https://codeforces.com/contest/{}/problem/{}",
            problem.contest_id, problem.index
        ),
        id_to_solved_count.get(&problem.contest_id).cloned(),
        None,
    )
}

fn build_contest(contest: CodeforcesContest, problems: Vec<Problem>) -> Contest {
    Contest::reconstruct(
        contest.id.to_string(),
        contest.name.to_string(),
        String::from(classify_contest(&contest)),
        Platform::Codeforces,
        String::from(contest.phase),
        if contest.start_time_seconds.is_some() {
            Some(contest.start_time_seconds.unwrap() as u64)
        } else {
            None
        },
        Some(contest.duration_seconds),
        problems,
    )
}
