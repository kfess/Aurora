use crate::{
    domain::{
        contest::Contest,
        problem::Problem,
        vo::{phase::Phase, platform::Platform},
    },
    infra::api::{api_client::ApiClient, yoj::external::ProblemCategories},
    utils::{api::get_toml, format::num_to_alphabet},
};
use anyhow::Result;
use convert_case::{Case, Casing};

use super::classifier::classify_contest;

const CATEGORY_TOML_URL: &'static str =
    "https://raw.githubusercontent.com/yosupo06/library-checker-problems/master/categories.toml";

pub trait YOJAPIClient: Send + Sync {
    async fn get_yoj_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)>;
}

impl ApiClient {
    async fn fetch_yoj_categories(&self) -> Result<ProblemCategories> {
        let categories = get_toml(CATEGORY_TOML_URL, &self.client).await?;
        Ok(categories)
    }

    async fn build_yoj_problems_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let raw_categories = self.fetch_yoj_categories().await?;

        let mut problems: Vec<Problem> = vec![];
        let mut contests: Vec<Contest> = vec![];
        for category in raw_categories.categories.iter() {
            let mut tmp_problems: Vec<Problem> = vec![];
            for (index, raw_problem) in category.raw_problems.iter().enumerate() {
                let problem = build_problem(&category.name, index, raw_problem);
                problems.push(problem.clone());
                tmp_problems.push(problem);
            }

            contests.push(build_contest(&category.name, tmp_problems));
        }

        Ok((problems, contests))
    }
}

impl YOJAPIClient for ApiClient {
    async fn get_yoj_problems_and_contests(&self) -> Result<(Vec<Problem>, Vec<Contest>)> {
        let (problems, contests) = self.build_yoj_problems_contests().await?;

        Ok((problems, contests))
    }
}

fn build_problem(category_name: &str, index: usize, raw_problem: &str) -> Problem {
    Problem::reconstruct(
        Platform::YOJ,
        &category_name,
        &num_to_alphabet(index),
        raw_problem.to_case(Case::Title).as_str(),
        Option::None,
        Option::None,
        Option::None,
        vec![],
        &format!("https://judge.yosupo.jp/problem/{raw_problem}"),
        Option::None,
        Option::None,
    )
}

fn build_contest(category_name: &str, problems: Vec<Problem>) -> Contest {
    Contest::reconstruct(
        category_name.to_string(),
        category_name.to_string(),
        String::from(classify_contest(category_name)),
        Platform::YOJ,
        String::from(Phase::Finished),
        Option::None,
        Option::None,
        problems,
    )
}
