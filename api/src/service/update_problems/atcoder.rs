use crate::infra::api::factory::APIClientFactoryTrait;

pub struct FetchAtcoderUsecase<T: APIClientFactoryTrait> {
    api_client_factory: T,
}

impl<T: APIClientFactoryTrait> FetchAtcoderUsecase<T> {
    pub fn new(api_client_factory: T) -> Self {
        Self { api_client_factory }
    }

    pub async fn execute(&self) {
        log::info!("AtCoder: update problems and contests");

        let atcoder_client = self.api_client_factory.get_atcoder_client().await.unwrap();
        let subs = atcoder_client.get_recent_submissions().await.unwrap();
        for sub in subs {
            println!("{:?}", sub);
        }
    }
}
