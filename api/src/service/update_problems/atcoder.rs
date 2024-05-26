use crate::infra::{
    api::factory::APIClientFactoryTrait, repository::problem::ProblemRepositoryTrait,
};

pub struct UpdateAtcoderUsecase<T: APIClientFactoryTrait, P: ProblemRepositoryTrait> {
    api_client_factory: T,
    repository: P,
}

impl<T: APIClientFactoryTrait, P: ProblemRepositoryTrait> UpdateAtcoderUsecase<T, P> {
    pub fn new(api_client_factory: T, repository: P) -> Self {
        Self {
            api_client_factory,
            repository,
        }
    }

    /// Fetch Atocoder problems and contests and update the database.
    /// This method is called periodically by the scheduler.
    pub async fn fetch_and_update(&self) {
        log::info!("AtCoder: update problems and contests");

        let atcoder_client = self.api_client_factory.get_atcoder_client().await.unwrap();

        let problems = atcoder_client.get_problems().await.unwrap();
        self.repository
            .update_problems("atcoder", &problems)
            .await
            .unwrap();
    }
}
