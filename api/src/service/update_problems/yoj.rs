use crate::infra::api::yoj::api_client::IYOJAPIClient;

pub struct UpdateYOJUsecase<T>
where
    T: IYOJAPIClient,
{
    api_client: T,
}

impl<T: IYOJAPIClient> UpdateYOJUsecase<T> {
    pub fn new(api_client: T) -> Self {
        Self { api_client }
    }

    pub async fn execute(&self) {
        let contests = self.api_client.get_contests().await;
        for contest in contests.iter() {
            println!("{:?}", contest);
        }
    }
}
