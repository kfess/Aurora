use crate::infra::api::factory::APIClientFactoryTrait;
use crate::infra::repository::problem::ProblemRepositoryTrait;

pub struct UpdateAojUsecase<T, P>
where
    T: APIClientFactoryTrait,
    P: ProblemRepositoryTrait,
{
    api_client_factory: T,
    repository: P,
}

impl<T, P> UpdateAojUsecase<T, P>
where
    T: APIClientFactoryTrait,
    P: ProblemRepositoryTrait,
{
    pub fn new(api_client_factory: T, repository: P) -> Self {
        return Self {
            api_client_factory,
            repository,
        };
    }

    pub async fn fetch_and_update(&self) {
        log::info!("Aizu Online Judge: update problems and contests");

        let aoj_client = self.api_client_factory.get_aoj_client().await.unwrap();

        let problems = aoj_client.get_problems().await.unwrap();
        self.repository
            .update_problems("aoj", &problems)
            .await
            .unwrap();
    }
}
