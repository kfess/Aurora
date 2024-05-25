use anyhow::{Ok, Result};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    domain::{
        contest::Contest,
        problem::Problem,
        submission::Submission,
        vo::{
            phase::Phase,
            platform::{self, Platform},
            verdict::Verdict,
        },
    },
    utils::api::get_json,
};

use super::{
    classifier::classify_contest,
    external::{AtcoderContest, AtcoderProblem, AtcoderSubmission, Estimation},
};

const ATCODER_INFORMATION_URL: &'static str = "https://kenkoooo.com/atcoder/resources";
const ATCODER_STATISTICS_URL: &'static str = "https://kenkoooo.com/atcoder/atcoder-api/v3";

/// AtCoder API Client
/// This struct is used to fetch data provided by Unofficial AtCoder API (kenkoooo)
pub struct AtcoderAPIClient {
    client: Arc<reqwest::Client>,
    cache: RwLock<Option<(Vec<Problem>, Vec<Contest>)>>,
}

#[async_trait]
pub trait AtcoderAPIClientTrait: Send + Sync {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_contests(&self) -> Result<Vec<Contest>>;
    async fn get_recent_submissions(&self) -> Result<Vec<Submission>>;
    async fn get_user_submissions(
        &self,
        user: &str,
        from_second: Option<u64>,
    ) -> Result<Vec<Submission>>;
}

impl AtcoderAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            cache: RwLock::new(None),
        }
    }

    async fn fetch_contests(&self) -> Result<Vec<AtcoderContest>> {
        let url = format!("{ATCODER_INFORMATION_URL}/contests.json");
        let contests = get_json::<Vec<AtcoderContest>>(&url, &self.client).await?;

        Ok(contests)
    }

    async fn fetch_problems(&self) -> Result<Vec<AtcoderProblem>> {
        let url = format!("{ATCODER_INFORMATION_URL}/merged-problems.json");
        let problems = get_json::<Vec<AtcoderProblem>>(&url, &self.client).await?;

        Ok(problems)
    }

    async fn fetch_estimations(&self) -> Result<HashMap<String, Estimation>> {
        let url = format!("{ATCODER_INFORMATION_URL}/problem-models.json");
        let estimations = get_json::<HashMap<String, Estimation>>(&url, &self.client).await?;

        Ok(estimations)
    }

    /// Fetch the most recent submissions from the AtCoder API.
    /// Retrieves up to 1,000 of the latest submissions.
    async fn fetch_recent_submissions(&self) -> Result<Vec<AtcoderSubmission>> {
        let url = format!("{ATCODER_STATISTICS_URL}/recent");
        let submissions = get_json::<Vec<AtcoderSubmission>>(&url, &self.client).await?;

        Ok(submissions)
    }

    async fn fetch_user_submissions(
        &self,
        user: &str,
        from_second: Option<u64>,
    ) -> Result<Vec<AtcoderSubmission>> {
        let url = format!(
            "{}/user/submissions?user={}&from_second={}",
            ATCODER_STATISTICS_URL,
            user,
            from_second.unwrap_or(0)
        );
        let submissions = get_json::<Vec<AtcoderSubmission>>(&url, &self.client).await?;

        Ok(submissions)
    }

    async fn build_problems_contests(&self) -> Result<()> {
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

            let problem = build_problem(p, diff, is_experimental);
            c_to_p_map
                .entry(p.contest_id.clone())
                .or_insert_with(Vec::new)
                .push(problem.clone());
            problems.push(problem);
        });

        let raw_contests = self.fetch_contests().await?;
        let mut contests: Vec<Contest> = vec![];
        raw_contests.iter().for_each(|c| {
            let contest = build_contest(c, c_to_p_map.get(&c.id).unwrap());
            contests.push(contest);
        });

        *self.cache.write().unwrap() = Some((problems.clone(), contests.clone()));

        Ok(())
    }
}

#[async_trait]
impl AtcoderAPIClientTrait for AtcoderAPIClient {
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

    async fn get_recent_submissions(&self) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_recent_submissions().await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect();

        Ok(submissions)
    }

    async fn get_user_submissions(
        &self,
        user: &str,
        from_second: Option<u64>,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_user_submissions(user, from_second).await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect();

        Ok(submissions)
    }
}

fn build_problem(
    p: &AtcoderProblem,
    difficulty: Option<f64>,
    is_experimental: Option<bool>,
) -> Problem {
    Problem::reconstruct(
        p.contest_id.to_string(),
        p.problem_index.to_string(),
        p.name.to_string(),
        platform::Platform::Atcoder,
        Some(p.point),
        difficulty,
        is_experimental,
        vec![],
        format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            p.contest_id, p.problem_index
        ),
        Some(p.solver_count),
        None,
    )
}

fn build_contest(c: &AtcoderContest, ps: &Vec<Problem>) -> Contest {
    Contest::reconstruct(
        c.id.to_string(),
        c.title.to_string(),
        String::from(classify_contest(&c)),
        platform::Platform::Atcoder,
        String::from(Phase::Finished),
        Some(c.start_epoch_second),
        Some(c.duration_second),
        ps.clone(),
    )
}

fn build_submission(s: &AtcoderSubmission) -> Submission {
    // Atcoder API does not provide problem name, point, and difficulty.
    // So, we have to set them at the front-end.
    Submission::reconstruct(
        Platform::Atcoder,
        s.id.to_string(),
        s.user_id.clone(),
        s.language.clone(),
        Verdict::from(s.result.as_str()),
        s.execution_time,
        None,
        Some(s.length),
        s.epoch_second,
        Some(s.contest_id.clone()),
        Some(s.problem_id.clone()),
        None,
        None,
        None,
    )
}
