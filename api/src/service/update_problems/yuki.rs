use crate::infra::api::yuki::api_client::YukicoderAPIClient;

pub struct UpdateYukicoderUsecase<C: YukicoderAPIClient> {
    api_client: C,
}

impl<C: YukicoderAPIClient> UpdateYukicoderUsecase<C> {
    pub fn new(api_client: C) -> Self {
        Self { api_client }
    }

    pub async fn execute(&self, is_recent: bool) {
        log::info!("Yukicoder: update problems and contests");

        let (problems, contests) = self
            .api_client
            .get_yuki_problems_and_contests()
            .await
            .unwrap();
    }
}
