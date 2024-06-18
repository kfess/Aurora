use anyhow::{Ok, Result};
use std::collections::HashMap;

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
use crate::infra::api::api_client::ApiClient;

const ATCODER_INFORMATION_URL: &'static str = "https://kenkoooo.com/atcoder/resources";
const ATCODER_STATISTICS_URL: &'static str = "https://kenkoooo.com/atcoder/atcoder-api/v3";

#[trait_variant::make]
pub trait AtcoderAPIClient: Send + Sync {
    async fn get_atcoder_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)>;
    async fn get_atcoder_recent_submissions(&self) -> Result<Vec<Submission>>;
    async fn get_atcoder_user_submissions(
        &self,
        user: &str,
        from_second: Option<u64>,
    ) -> Result<Vec<Submission>>;
}

impl ApiClient {
    async fn fetch_atcoder_contests(&self) -> Result<Vec<AtcoderContest>> {
        let url = format!("{ATCODER_INFORMATION_URL}/contests.json");
        let contests = get_json::<Vec<AtcoderContest>>(&url, &self.client).await?;

        Ok(contests)
    }

    async fn fetch_atcoder_problems(&self) -> Result<Vec<AtcoderProblem>> {
        let url = format!("{ATCODER_INFORMATION_URL}/merged-problems.json");
        let problems = get_json::<Vec<AtcoderProblem>>(&url, &self.client).await?;

        Ok(problems)
    }

    async fn fetch_atcoder_estimations(&self) -> Result<HashMap<String, Estimation>> {
        let url = format!("{ATCODER_INFORMATION_URL}/problem-models.json");
        let estimations = get_json::<HashMap<String, Estimation>>(&url, &self.client).await?;

        Ok(estimations)
    }

    /// Fetch the most recent submissions from the AtCoder API.
    /// Retrieves up to 1,000 of the latest submissions.
    async fn fetch_atcoder_recent_submissions(&self) -> Result<Vec<AtcoderSubmission>> {
        let url = format!("{ATCODER_STATISTICS_URL}/recent");
        let submissions = get_json::<Vec<AtcoderSubmission>>(&url, &self.client).await?;

        Ok(submissions)
    }

    async fn fetch_atcoder_user_submissions(
        &self,
        user: &str,
        from_second: Option<u64>,
    ) -> Result<Vec<AtcoderSubmission>> {
        let url = format!(
            "{ATCODER_STATISTICS_URL}/user/submissions?user={user}&from_second={}",
            from_second.unwrap_or(0)
        );
        let submissions = get_json::<Vec<AtcoderSubmission>>(&url, &self.client).await?;

        Ok(submissions)
    }

    async fn build_atcoder_problems_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let estimations = self.fetch_atcoder_estimations().await?;

        let mut c_to_p_map: HashMap<String, Vec<Problem>> = HashMap::new();
        let raw_problems = self.fetch_atcoder_problems().await?;
        let mut problems: Vec<Problem> = vec![];
        raw_problems.iter().for_each(|p| {
            let (diff, is_experimental) = clip_difficulty(estimations.get(&p.id));
            let problem = build_problem(p, diff, is_experimental);
            c_to_p_map
                .entry(p.contest_id.clone())
                .or_insert_with(Vec::new)
                .push(problem.clone());
            problems.push(problem);
        });

        let raw_contests = self.fetch_atcoder_contests().await?;
        let mut contests: Vec<Contest> = vec![];
        raw_contests.iter().for_each(|c| {
            let contest = build_contest(c, c_to_p_map.get(&c.id).unwrap_or(&vec![]));
            contests.push(contest);
        });

        Ok((problems, contests))
    }
}

impl AtcoderAPIClient for ApiClient {
    async fn get_atcoder_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let (problems, contests) = self.build_atcoder_problems_contests().await?;

        Ok((problems, contests))
    }

    async fn get_atcoder_recent_submissions(&self) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_atcoder_recent_submissions().await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect();

        Ok(submissions)
    }

    async fn get_atcoder_user_submissions(
        &self,
        user: &str,
        from_second: Option<u64>,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self
            .fetch_atcoder_user_submissions(user, from_second)
            .await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect();

        Ok(submissions)
    }
}

fn clip_difficulty(estimation: Option<&Estimation>) -> (Option<f64>, Option<bool>) {
    match estimation {
        Some(estimation) => {
            if let Some(difficulty) = estimation.difficulty {
                let clipped_diff = if difficulty >= 400.0 {
                    difficulty.round()
                } else {
                    (400.0 / f64::exp(1.0 - difficulty / 400.0)).round()
                };
                (Some(clipped_diff), estimation.is_experimental)
            } else {
                (None, estimation.is_experimental)
            }
        }
        None => (None, None),
    }
}

fn build_problem(
    p: &AtcoderProblem,
    difficulty: Option<f64>,
    is_experimental: Option<bool>,
) -> Problem {
    Problem::reconstruct(
        platform::Platform::Atcoder,
        &p.contest_id,
        &p.problem_index,
        &p.name,
        p.point,
        difficulty,
        is_experimental,
        vec![],
        &format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            p.contest_id, p.problem_index
        ),
        p.solver_count,
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
