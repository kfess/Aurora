use crate::infra::api::yoj::api_client::YOJAPIClient;

pub struct UpdateYOJUsecase<C>
where
    C: YOJAPIClient,
{
    api_client: C,
}

impl<C: YOJAPIClient> UpdateYOJUsecase<C> {
    pub fn new(api_client: C) -> Self {
        Self { api_client }
    }

    pub async fn execute(&self) {
        let (problems, contests) = self
            .api_client
            .get_yoj_problems_and_contests()
            .await
            .unwrap();

        for contest in contests.iter() {
            println!("{:?}", contest);
        }
    }
}
