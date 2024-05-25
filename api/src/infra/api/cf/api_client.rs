use anyhow::{Ok, Result};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use url::Url;

use crate::{
    domain::{
        contest::Contest,
        problem::Problem,
        submission::Submission,
        vo::{platform::Platform, verdict::Verdict},
    },
    utils::api::get_json,
};

use super::{
    classifier::classify_contest,
    external::{
        CodeforcesContest, CodeforcesContestResponse, CodeforcesProblem, CodeforcesProblemResponse,
        CodeforcesProblemStat, CodeforcesSubmission, CodeforcesSubmissionResponse,
    },
};

const CODEFORCES_URL_PREFIX: &'static str = "https://codeforces.com/api";

pub struct CFAPIClient {
    client: Arc<reqwest::Client>,
    cache: RwLock<Option<(Vec<Problem>, Vec<Contest>)>>,
}

#[async_trait]
pub trait CFAPIClientTrait: Send + Sync {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_contests(&self) -> Result<Vec<Contest>>;
    async fn get_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<Submission>>;
    async fn get_recent_submissions(&self) -> Result<Vec<Submission>>;
}

impl CFAPIClient {
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

    async fn fetch_recent_submissions(&self) -> Result<Vec<CodeforcesSubmission>> {
        let url = format!("{CODEFORCES_URL_PREFIX}/problemset.recentStatus?count=100");
        let result = get_json::<CodeforcesSubmissionResponse>(&url, &self.client).await?;
        let submissions = result.result.unwrap();

        Ok(submissions)
    }

    async fn fetch_user_submissions(
        &self,
        user_id: &str,
        from: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<CodeforcesSubmission>> {
        // let mut url = Url::parse("https://codeforces.com/api/user.status").unwrap();
        let mut url = Url::parse(&format!(
            "{CODEFORCES_URL_PREFIX}/user.status?handle={user_id}"
        ))
        .unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("handle", user_id);
            if let Some(from) = from {
                query_pairs.append_pair("from", &from.to_string());
            }
            if let Some(count) = count {
                query_pairs.append_pair("count", &count.to_string());
            }
        }

        let result = get_json::<CodeforcesSubmissionResponse>(&url.as_str(), &self.client).await?;
        let submissions = result.result.unwrap();

        Ok(submissions)
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

#[async_trait]
impl CFAPIClientTrait for CFAPIClient {
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

    async fn get_user_submissions(
        &self,
        user_id: &str,
        from: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_user_submissions(user_id, from, count).await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect::<Vec<Submission>>();

        Ok(submissions)
    }

    async fn get_recent_submissions(&self) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_recent_submissions().await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect::<Vec<Submission>>();

        Ok(submissions)
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

fn build_submission(s: &CodeforcesSubmission) -> Submission {
    Submission::reconstruct(
        Platform::Codeforces,
        s.id.to_string(),
        s.author.members[0].handle.clone(),
        s.programming_language.clone(),
        Verdict::from(s.verdict.as_str()),
        Some(s.time_consumed_millis),
        Some(s.memory_consumed_bytes / 1024),
        None,
        s.creation_time_seconds,
        Some(s.contest_id.unwrap_or(0).to_string()),
        Some(s.problem.index.clone()),
        Some(s.problem.name.clone()),
        s.problem.points,
        s.problem.rating,
    )
}
