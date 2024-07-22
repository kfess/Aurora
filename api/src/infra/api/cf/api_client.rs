use anyhow::{Ok, Result};
use std::collections::HashMap;
use url::Url;

use crate::{
    domain::{
        contest::Contest,
        problem::Problem,
        submission::Submission,
        vo::{platform::Platform, verdict::Verdict},
    },
    infra::api::api_client::ApiClient,
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

#[trait_variant::make]
pub trait CFAPIClient: Send + Sync {
    async fn get_cf_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)>;
    async fn get_cf_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<Submission>>;
    async fn get_cf_recent_submissions(&self) -> Result<Vec<Submission>>;
}

impl ApiClient {
    async fn fetch_cf_problems(
        &self,
    ) -> Result<(Vec<CodeforcesProblem>, Vec<CodeforcesProblemStat>)> {
        let url = format!("{CODEFORCES_URL_PREFIX}/problemset.problems");
        let result = get_json::<CodeforcesProblemResponse>(&url, &self.client).await?;
        let problems_with_stats = result.result.unwrap();

        Ok((
            problems_with_stats.problems,
            problems_with_stats.problem_statistics,
        ))
    }

    async fn fetch_cf_past_contests(&self) -> Result<Vec<CodeforcesContest>> {
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
    async fn fetch_cf_future_contests(&self) -> Result<Vec<CodeforcesContest>> {
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

    async fn fetch_cf_recent_submissions(&self) -> Result<Vec<CodeforcesSubmission>> {
        let url = format!("{CODEFORCES_URL_PREFIX}/problemset.recentStatus?count=100");
        let result = get_json::<CodeforcesSubmissionResponse>(&url, &self.client).await?;
        let submissions = result.result.unwrap();

        Ok(submissions)
    }

    async fn fetch_cf_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<CodeforcesSubmission>> {
        let mut url = Url::parse(&format!(
            "{CODEFORCES_URL_PREFIX}/user.status?handle={user_id}"
        ))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("handle", user_id);

            if let Some(page) = page {
                query_pairs
                    .append_pair("from", &((page - 1) * size.unwrap_or(100) + 1).to_string());
            }
            if let Some(size) = size {
                query_pairs.append_pair("count", &size.to_string());
            }
        }

        let result = get_json::<CodeforcesSubmissionResponse>(&url.as_str(), &self.client).await?;
        let submissions = result.result.unwrap();

        Ok(submissions)
    }

    async fn build_cf_problems_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let (raw_problems, raw_stats) = self.fetch_cf_problems().await?;
        let raw_contests = self.fetch_cf_past_contests().await?;

        let id_to_solved_count: HashMap<u64, i32> = raw_stats
            .iter()
            .map(|s| (s.contest_id, s.solved_count))
            .collect();

        let c_id_c_map: HashMap<u64, CodeforcesContest> = raw_contests
            .iter()
            .map(|c| (c.id.clone(), c.clone()))
            .collect();

        let mut c_to_p_map: HashMap<u64, Vec<Problem>> = HashMap::new();

        let mut problems: Vec<Problem> = vec![];
        raw_problems.iter().for_each(|p| {
            let raw_contest = c_id_c_map.get(&p.contest_id).unwrap();

            let problem = build_problem(p, raw_contest, id_to_solved_count.clone());
            c_to_p_map
                .entry(p.contest_id)
                .or_insert_with(Vec::new)
                .push(problem.clone());
            problems.push(problem);
        });

        let mut contests: Vec<Contest> = vec![];
        raw_contests.iter().for_each(|c| {
            let ps: Vec<Problem> = c_to_p_map.get(&c.id).unwrap_or(&vec![]).clone();
            contests.push(build_contest(c.clone(), ps));
        });

        Ok((problems, contests))
    }
}

impl CFAPIClient for ApiClient {
    async fn get_cf_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let (problems, contests) = self.build_cf_problems_contests().await?;

        Ok((problems, contests))
    }

    async fn get_cf_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_cf_user_submissions(user_id, page, size).await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect::<Vec<Submission>>();

        Ok(submissions)
    }

    async fn get_cf_recent_submissions(&self) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_cf_recent_submissions().await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect::<Vec<Submission>>();

        Ok(submissions)
    }
}

fn build_problem(
    problem: &CodeforcesProblem,
    contest: &CodeforcesContest,
    id_to_solved_count: HashMap<u64, i32>,
) -> Problem {
    Problem::reconstruct(
        Platform::Codeforces,
        &problem.contest_id.to_string().as_str(),
        &problem.index,
        &problem.name,
        problem.points,
        problem.rating,
        String::from(classify_contest(&contest)),
        Some(false),
        problem
            .tags
            .iter()
            .map(|t| t.trim().to_string())
            .collect::<Vec<String>>(),
        &format!(
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
            Some(contest.start_time_seconds.unwrap() as i64)
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
