use crate::infra::{
    api::aoj::api_client::AojAPIClient, repository::problem::ProblemRepositoryTrait,
};

pub struct UpdateAojUsecase<C, R>
where
    C: AojAPIClient,
    R: ProblemRepositoryTrait,
{
    api_client: C,
    repository: R,
}

impl<C, R> UpdateAojUsecase<C, R>
where
    C: AojAPIClient,
    R: ProblemRepositoryTrait,
{
    pub fn new(api_client: C, repository: R) -> Self {
        return Self {
            api_client,
            repository,
        };
    }

    pub async fn fetch_and_update(&self) {
        log::info!("Aizu Online Judge: update problems and contests");

        let (problems, _contests) = self
            .api_client
            .get_aoj_problems_and_contests()
            .await
            .unwrap();
        self.repository
            .update_problems("aoj", &problems)
            .await
            .unwrap();
    }
}
