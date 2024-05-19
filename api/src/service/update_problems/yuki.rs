use crate::infra::api::factory::APIClientFactoryTrait;

pub struct UpdateYukicoderUsecase<T: APIClientFactoryTrait> {
    api_client_factory: T,
}

impl<T: APIClientFactoryTrait> UpdateYukicoderUsecase<T> {
    pub fn new(api_client_factory: T) -> Self {
        Self { api_client_factory }
    }

    pub async fn execute(&self, is_recent: bool) {
        log::info!("Yukicoder: update problems and contests");

        let yuki_client = self.api_client_factory.get_yuki_client().await.unwrap();
        let problems = yuki_client.get_problems(is_recent).await.unwrap();
        for problem in problems {
            println!("{:?}", problem);
        }

        let contests = yuki_client.get_contests(is_recent).await.unwrap();
        for contest in contests.iter() {
            println!("{:?}", contest);
        }
    }
}
