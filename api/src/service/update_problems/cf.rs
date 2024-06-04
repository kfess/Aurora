use crate::infra::{
    api::cf::api_client::CFAPIClient,
    repository::{self, problem::ProblemRepository},
};
use anyhow::{Context, Result};

pub struct UpdateCodeforcesUsecase<C, R>
where
    C: CFAPIClient,
    R: ProblemRepository,
{
    api_client: C,
    repository: R,
}

impl<C: CFAPIClient, R: ProblemRepository> UpdateCodeforcesUsecase<C, R> {
    pub fn new(api_client: C, repository: R) -> Self {
        return Self {
            api_client,
            repository,
        };
    }

    pub async fn fetch_and_update(&self) {
        log::info!("Codeforces: update problems and contests");

        let (problems, _contests) = self
            .api_client
            .get_cf_problems_and_contests()
            .await
            .with_context(|| "Failed to fetch Codeforces problems")
            .unwrap();

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| "Failed to update Codeforces problems")
            .unwrap();
    }
}
