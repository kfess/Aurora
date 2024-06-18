use anyhow::Result;

use crate::infra::repository::problem::Condition;
use crate::{domain::problem::Problem, infra::repository::problem::ProblemRepository};

pub struct FetchProblemUsecase<R>
where
    R: ProblemRepository,
{
    repository: R,
}

#[trait_variant::make]
pub trait FetchProblem {
    async fn fetch_problems(&self, condition: &Condition<'_>) -> Result<Vec<Problem>>;
}

impl<R> FetchProblemUsecase<R>
where
    R: ProblemRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> FetchProblem for FetchProblemUsecase<R>
where
    R: ProblemRepository,
{
    async fn fetch_problems(&self, condition: &Condition<'_>) -> Result<Vec<Problem>> {
        self.repository.get_problems_by_condition(condition).await
    }
}
