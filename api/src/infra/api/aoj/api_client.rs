use self::types::{AojProblem, AojSubmission};
use crate::domain::vo::platform::Platform;
use crate::domain::vo::verdict::Verdict;
use crate::domain::{problem::Problem, submission::Submission};
use crate::utils::api::get_json;
use anyhow::{Ok, Result};
use std::sync::Arc;

use super::*;

const AOJ_URL: &str = "https://judgeapi.u-aizu.ac.jp";

pub struct AojAPIClient {
    client: Arc<reqwest::Client>,
}

impl AojAPIClient {
    pub fn new() -> Self {
        return Self {
            client: Arc::new(reqwest::Client::new()),
        };
    }

    async fn fetch_problems(&self) -> Result<Vec<AojProblem>> {
        const SIZE: u32 = 10000;
        let url = format!("{}/problems?size={}", AOJ_URL, SIZE);
        let problems: Vec<AojProblem> = get_json(&url, &self.client).await?;

        Ok(problems)
    }

    async fn fetch_user_submission(
        &self,
        user_id: &str,
        page: u32,
        size: u32,
    ) -> Result<Vec<AojSubmission>> {
        let url = format!(
            "{}/submission_records/users/{}?page={}&size={}",
            AOJ_URL, user_id, page, size
        );
        let submissions: Vec<AojSubmission> = get_json(&url, &self.client).await?;

        Ok(submissions)
    }
}

pub trait IAojAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_user_submission(
        &self,
        user_id: &str,
        page: u32,
        size: u32,
    ) -> Result<Vec<Submission>>;
}

impl IAojAPIClient for AojAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        Ok(vec![])
    }

    async fn get_user_submission(
        &self,
        user_id: &str,
        page: u32,
        size: u32,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_user_submission(user_id, page, size).await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| {
                Submission::reconstruct(
                    s.judge_id,
                    s.language.clone(),
                    Platform::Aoj,
                    map_status_to_verdict(s.status),
                    Some(s.memory),
                    Some(s.cpu_time),
                    s.code_size,
                    s.submission_date,
                    s.problem_id.clone(),
                    None,
                )
            })
            .collect();

        Ok(submissions)
    }
}

fn map_status_to_verdict(status: u16) -> Verdict {
    // http://developers.u-aizu.ac.jp/index
    match status {
        0 => Verdict::CompileError,
        1 => Verdict::WrongAnswer,
        2 => Verdict::TimeLimitExceeded,
        3 => Verdict::MemoryLimitExceeded,
        4 => Verdict::Accepted,
        5 => Verdict::Waiting,
        6 => Verdict::OutputLimit,
        7 => Verdict::RuntimeError,
        8 => Verdict::PresentationError,
        9 => Verdict::RuntimeError,
        _ => Verdict::Unknown,
    }
}
