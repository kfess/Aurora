use std::sync::Arc;

use crate::{domain::problem::Problem, infra::repository::problem::ProblemRepository};

struct FetchProblemUsecase<R>
where
    R: ProblemRepository,
{
    repository: Arc<R>,
}

// pub trait FetchProblemUsecase

impl<R> FetchProblemUsecase<R>
where
    R: ProblemRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub fn fetch_problems(&self) -> Vec<Problem> {
        vec![]
    }
}
