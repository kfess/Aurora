use crate::infra::{
    api::atcoder::api_client::AtcoderAPIClient, repository::problem::ProblemRepositoryTrait,
};

pub struct UpdateAtcoderUsecase<C: AtcoderAPIClient, R: ProblemRepositoryTrait> {
    api_client: C,
    repository: R,
}

impl<C: AtcoderAPIClient, R: ProblemRepositoryTrait> UpdateAtcoderUsecase<C, R> {
    pub fn new(api_client: C, repository: R) -> Self {
        Self {
            api_client,
            repository,
        }
    }

    /// Fetch Atocoder problems and contests and update the database.
    /// This method is called periodically by the scheduler.
    pub async fn fetch_and_update(&self) {
        log::info!("AtCoder: update problems and contests");

        let (problems, _contests) = self
            .api_client
            .get_atcoder_problems_and_contests()
            .await
            .unwrap();

        self.repository
            .update_problems("atcoder", &problems)
            .await
            .unwrap();
    }
}
