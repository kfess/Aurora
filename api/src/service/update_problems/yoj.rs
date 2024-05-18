use crate::infra::api::yoj::api_client::YOJAPIClientTrait;

pub struct UpdateYOJUsecase<T>
where
    T: YOJAPIClientTrait,
{
    api_client: T,
}

impl<T: YOJAPIClientTrait> UpdateYOJUsecase<T> {
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
