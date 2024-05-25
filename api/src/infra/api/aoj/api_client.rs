use self::external::{
    AojChallenges, AojChallengesAndRelatedContests, AojProblem, AojSubmission, AojVolume,
    AojVolumesChallengesList,
};
use crate::domain::vo::platform::Platform;
use crate::domain::vo::verdict::Verdict;
use crate::domain::{problem::Problem, submission::Submission};
use crate::utils::api::get_json;
use anyhow::{Ok, Result};
use async_trait::async_trait;
use std::sync::Arc;
use url::Url;

use super::*;

const AOJ_URL: &'static str = "https://judgeapi.u-aizu.ac.jp";

pub struct AojAPIClient {
    client: Arc<reqwest::Client>,
}

#[async_trait]
pub trait AojAPIClientTrait: Send + Sync {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<Submission>>;
    async fn get_recent_submissions(&self) -> Result<Vec<Submission>>;
}

impl AojAPIClient {
    pub fn new() -> Self {
        return Self {
            client: Arc::new(reqwest::Client::new()),
        };
    }

    async fn fetch_problems(&self) -> Result<Vec<AojProblem>> {
        const SIZE: u32 = 10000;
        let url = format!("{AOJ_URL}/problems?size={SIZE}");
        let problems: Vec<AojProblem> = get_json(&url, &self.client).await?;

        Ok(problems)
    }

    async fn fetch_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<AojSubmission>> {
        let mut url = Url::parse(&format!("{AOJ_URL}/submission_records/users/{user_id}")).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(page) = page {
                query_pairs.append_pair("page", &page.to_string());
            }
            if let Some(size) = size {
                query_pairs.append_pair("size", &size.to_string());
            }
        }

        let submissions: Vec<AojSubmission> = get_json(&url.as_str(), &self.client).await?;

        Ok(submissions)
    }

    async fn fetch_recent_submissions(&self) -> Result<Vec<AojSubmission>> {
        let url = format!("{AOJ_URL}/submission_records/recent");
        let submissions: Vec<AojSubmission> = get_json(&url, &self.client).await?;

        Ok(submissions)
    }

    async fn get_volumes_challenges_list(&self) -> Result<(Vec<u16>, Vec<String>)> {
        let url = format!("{AOJ_URL}/problems/filters");
        let list: AojVolumesChallengesList = get_json(&url, &self.client).await?;

        let (volume_ids, large_cls) = (list.volumes, list.large_cls);

        Ok((volume_ids, large_cls))
    }

    async fn get_large_cls_middle_cls(&self) -> Result<Vec<(String, String)>> {
        let url = format!("{AOJ_URL}/challenges");
        let challenges: AojChallenges = get_json(&url, &self.client).await?;

        let pairs: Vec<(String, String)> = challenges
            .large_cls
            .iter()
            .flat_map(|l| {
                l.middle_cls
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(move |m| (l.id.clone(), m.id.clone()))
            })
            .collect();

        Ok(pairs)
    }

    async fn get_problems_by_volume_id(&self, volume_id: u16) -> Result<Vec<AojProblem>> {
        let url = format!("{AOJ_URL}/problems/volumes/{volume_id}");
        let volume: AojVolume = get_json(&url, &self.client).await?;
        let problems = volume.problems;

        Ok(problems)
    }

    async fn get_challenges_by_large_cl_middle_cl(
        &self,
        large_cl: &str,
        middle_cl: &str,
    ) -> Result<Vec<AojProblem>> {
        let url = format!("{AOJ_URL}/challenges/cl/{large_cl}/{middle_cl}");
        let chanllenges: AojChallengesAndRelatedContests = get_json(&url, &self.client).await?;

        Ok(vec![])
    }
}

#[async_trait]
impl AojAPIClientTrait for AojAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        Ok(vec![])
    }

    async fn get_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_user_submissions(user_id, page, size).await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect();

        Ok(submissions)
    }

    async fn get_recent_submissions(&self) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_recent_submissions().await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
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

fn build_submission(s: &AojSubmission) -> Submission {
    Submission::reconstruct(
        Platform::Aoj,
        s.judge_id.to_string(),
        s.user_id.clone(),
        s.language.clone(),
        Verdict::from(map_status_to_verdict(s.status)),
        Some(s.cpu_time * 10),
        Some(s.memory),
        Some(s.code_size),
        s.submission_date / 1000,
        None,
        Some(s.problem_id.clone()),
        s.problem_title.clone(),
        None,
        None,
    )
}
