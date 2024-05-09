use anyhow::Result;
use convert_case::{Case, Casing};
use std::{
    sync::{Arc, RwLock},
    vec,
};

use crate::{
    domain::{contest::Contest, problem::Problem, value_object::platform::Platform},
    infra::api::yoj::types::ProblemCategories,
    utils::format::num_to_alphabet,
};

const CATEGORY_TOML_URL: &'static str =
    "https://raw.githubusercontent.com/yosupo06/library-checker-problems/master/categories.toml";

pub struct YOJAPIClient {
    client: Arc<reqwest::Client>,
    cache: RwLock<Option<(Vec<Problem>, Vec<Contest>)>>,
}

impl YOJAPIClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            cache: RwLock::new(None),
        }
    }

    async fn fetch_categories(&self) -> Result<ProblemCategories> {
        let response = self.client.get(CATEGORY_TOML_URL).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch yosupo online judge categories"
            ));
        }

        let raw_toml = match response.text().await {
            Ok(raw_toml) => raw_toml,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to fetch yosupo online judge categories toml: {}",
                    e
                ))
            }
        };

        let categories = match toml::from_str::<ProblemCategories>(&raw_toml) {
            Ok(categories) => categories,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to parse yosupo online judge categories toml: {}",
                    e
                ))
            }
        };

        Ok(categories)
    }

    fn build_problem(&self, category_name: &str, index: usize, raw_problem: &str) -> Problem {
        Problem::reconstruct(
            category_name.to_string(),
            num_to_alphabet(index),
            raw_problem.to_string().to_case(Case::Title),
            Platform::YosupoOnlineJudge,
            Option::None,
            Option::None,
            vec![],
            format!("https://judge.yosupo.jp/problem/{}", raw_problem),
            Option::None,
            Option::None,
        )
    }

    fn build_contest(&self, category_name: &str, problems: Vec<Problem>) -> Contest {
        Contest::reconstruct(
            category_name.to_string(),
            Platform::YosupoOnlineJudge,
            "finished".to_string(),
            Option::None,
            Option::None,
            "https://judge.yosupo.jp/".to_string(),
            problems,
        )
    }

    async fn merge(&self) -> Result<()> {
        if self.cache.read().unwrap().is_some() {
            return Ok(());
        }

        let raw_categories = self.fetch_categories().await?;

        let mut problems: Vec<Problem> = vec![];
        let mut contests: Vec<Contest> = vec![];
        for category in raw_categories.categories.iter() {
            let mut tmp_problems: Vec<Problem> = vec![];
            for (index, raw_problem) in category.raw_problems.iter().enumerate() {
                problems.push(self.build_problem(&category.name, index, raw_problem));
                tmp_problems.push(self.build_problem(&category.name, index, raw_problem));
            }

            contests.push(self.build_contest(&category.name, tmp_problems));
        }

        *self.cache.write().unwrap() = Some((problems.clone(), contests.clone()));

        Ok(())
    }
}

pub trait IYOJAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>>;
    async fn get_contests(&self) -> Result<Vec<Contest>>;
}

impl IYOJAPIClient for YOJAPIClient {
    async fn get_problems(&self) -> Result<Vec<Problem>> {
        self.merge().await?;
        let cache = self.cache.read().unwrap();
        let (problems, _) = cache.as_ref().unwrap();

        Ok(problems.clone())
    }

    async fn get_contests(&self) -> Result<Vec<Contest>> {
        self.merge().await?;
        let cache = self.cache.read().unwrap();
        let (_, contests) = cache.as_ref().unwrap();

        Ok(contests.clone())
    }
}
