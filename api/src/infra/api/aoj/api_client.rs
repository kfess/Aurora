use self::external::{
    AojChallenges, AojChallengesAndRelatedContests, AojProblem, AojSubmission, AojVolume,
    AojVolumesChallengesList,
};
use crate::domain::vo::platform::Platform;
use crate::domain::vo::verdict::Verdict;
use crate::domain::{contest::Contest, problem::Problem, submission::Submission};
use crate::infra::api::api_client::ApiClient;
use crate::utils::api::get_json;
use anyhow::{Ok, Result};
use url::Url;

use super::*;

const AOJ_URL: &'static str = "https://judgeapi.u-aizu.ac.jp";

pub trait AojAPIClient: Send + Sync {
    async fn get_aoj_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)>;
    async fn get_aoj_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<Submission>>;
    async fn get_aoj_recent_submissions(&self) -> Result<Vec<Submission>>;
}

impl ApiClient {
    async fn fetch_aoj_user_submissions(
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

    async fn fetch_aoj_recent_submissions(&self) -> Result<Vec<AojSubmission>> {
        let url = format!("{AOJ_URL}/submission_records/recent");
        let submissions: Vec<AojSubmission> = get_json(&url, &self.client).await?;

        Ok(submissions)
    }

    async fn fetch_aoj_volumes_challenges_list(&self) -> Result<Vec<u16>> {
        let url = format!("{AOJ_URL}/problems/filters");
        let list: AojVolumesChallengesList = get_json(&url, &self.client).await?;

        let (volume_ids, _) = (list.volumes, list.large_cls);

        Ok(volume_ids)
    }

    async fn fetch_aoj_large_cls_middle_cls(&self) -> Result<Vec<(String, String)>> {
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

    async fn fetch_aoj_problems_by_volume_id(&self, volume_id: u16) -> Result<Vec<AojProblem>> {
        let url = format!("{AOJ_URL}/problems/volumes/{volume_id}");
        let volume: AojVolume = get_json(&url, &self.client).await?;
        let problems = volume.problems;

        Ok(problems)
    }

    async fn fetch_aoj_challenges_by_large_cl_middle_cl(
        &self,
        large_cl: &str,
        middle_cl: &str,
    ) -> Result<Vec<(u16, String, Vec<AojProblem>)>> {
        let url = format!("{AOJ_URL}/challenges/cl/{large_cl}/{middle_cl}");
        let chanllenges: AojChallengesAndRelatedContests = get_json(&url, &self.client).await?;

        // 年度と問題のペアを作成
        let mut pair: Vec<(u16, String, Vec<AojProblem>)> = vec![];
        for contest in chanllenges.contests {
            let year = contest.year;
            let title_problems = contest
                .days
                .iter()
                .map(|d| {
                    let title = d.title.clone();
                    let problems = d.problems.clone();
                    (title, problems)
                })
                .collect::<Vec<(String, Vec<AojProblem>)>>();

            for (title, problems) in title_problems {
                pair.push((year, title, problems));
            }
        }

        Ok(pair)
    }

    async fn build_aoj_problems_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let mut all_problems: Vec<Problem> = vec![];
        let mut all_contests: Vec<Contest> = vec![];

        // volume 内の問題を取得
        let volume_ids = self.fetch_aoj_volumes_challenges_list().await?;
        for vol_id in volume_ids {
            let raw_problems_in_vol = self.fetch_aoj_problems_by_volume_id(vol_id).await?;

            let problems_in_vol = raw_problems_in_vol
                .iter()
                .map(|p| build_problem_from_vol(vol_id, p))
                .collect::<Vec<Problem>>();

            let contest = build_contest_from_vol(vol_id, &problems_in_vol);
            all_contests.push(contest);
            all_problems.extend(problems_in_vol);
        }

        // large_cl, middle_cl から問題を取得
        let pairs = self.fetch_aoj_large_cls_middle_cls().await?;
        for pair in pairs {
            let raw_year_problems = self
                .fetch_aoj_challenges_by_large_cl_middle_cl(&pair.0, &pair.1)
                .await?;

            for (year, title, problems) in raw_year_problems {
                let problems = problems
                    .iter()
                    .map(|p| build_problen_from_cl(&pair.0, &pair.1, year, p))
                    .collect::<Vec<Problem>>();

                let contest = build_contest_from_cl(&pair.0, &pair.1, year, &title, &problems);
                all_contests.push(contest);
                all_problems.extend(problems);
            }
        }

        Ok((all_problems, all_contests))
    }
}

impl AojAPIClient for ApiClient {
    async fn get_aoj_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let (problems, contests) = self.build_aoj_problems_contests().await?;

        Ok((problems, contests))
    }

    async fn get_aoj_user_submissions(
        &self,
        user_id: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_aoj_user_submissions(user_id, page, size).await?;
        let submissions = raw_submissions
            .iter()
            .map(|s| build_submission(s))
            .collect();

        Ok(submissions)
    }

    async fn get_aoj_recent_submissions(&self) -> Result<Vec<Submission>> {
        let raw_submissions = self.fetch_aoj_recent_submissions().await?;
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

fn build_problem_from_vol(vol_id: u16, p: &AojProblem) -> Problem {
    Problem::reconstruct(
        Platform::Aoj,
        vol_id.to_string().as_str(),
        &p.id,
        &p.name,
        None,
        None,
        None,
        vec![],
        &format!(
            "https://onlinejudge.u-aizu.ac.jp/challenges/search/volumes/{}",
            p.id
        ),
        Some(p.solved_user),
        Some(p.submissions),
    )
}

fn build_problen_from_cl(large_cl: &str, middle_cl: &str, year: u16, p: &AojProblem) -> Problem {
    Problem::reconstruct(
        Platform::Aoj,
        &format!("{large_cl}_{middle_cl}_{year}"),
        &p.id,
        &p.name,
        None,
        None,
        None,
        vec![],
        &format!(
            "https://onlinejudge.u-aizu.ac.jp/challenges/sources/{}/{}/{}?year={}",
            large_cl, middle_cl, p.id, year
        ),
        Some(p.solved_user),
        Some(p.submissions),
    )
}

fn build_contest_from_vol(vol_id: u16, ps: &Vec<Problem>) -> Contest {
    Contest::reconstruct(
        format!("volume_{}", vol_id),
        format!("Volume {}", vol_id),
        format!("Volume {}", vol_id),
        Platform::Aoj,
        String::from("finished"),
        None,
        None,
        ps.clone(),
    )
}

fn build_contest_from_cl(
    large_cl: &str,
    middle_cl: &str,
    year: u16,
    title: &str,
    ps: &Vec<Problem>,
) -> Contest {
    Contest::reconstruct(
        format!("{large_cl}_{middle_cl}_{year}"),
        title.to_string(),
        String::from(large_cl),
        Platform::Aoj,
        String::from("finished"),
        None,
        None,
        ps.clone(),
    )
}
