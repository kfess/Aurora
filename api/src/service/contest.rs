use anyhow::Result;

use crate::infra::repository::contest::Condition;
use crate::{domain::contest::Contest, infra::repository::contest::ContestRepository};

pub struct FetchContestUsecase<R>
where
    R: ContestRepository,
{
    repository: R,
}

#[trait_variant::make]
pub trait FetchContest {
    async fn fetch_contests(&self, condition: &Condition<'_>) -> Result<Vec<Contest>>;
}

impl<R> FetchContestUsecase<R>
where
    R: ContestRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> FetchContest for FetchContestUsecase<R>
where
    R: ContestRepository,
{
    async fn fetch_contests(&self, condition: &Condition<'_>) -> Result<Vec<Contest>> {
        self.repository.get_contests_by_condition(condition).await
    }
}
