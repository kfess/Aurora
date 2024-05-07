use crate::infrastracture::api::yukicoder::api_client::IYukicoderAPIClient;

pub struct UpdateYukicoderProblemUsecase<T>
where
    T: IYukicoderAPIClient,
{
    api_client: T,
}

impl<T: IYukicoderAPIClient> UpdateYukicoderProblemUsecase<T> {
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub async fn execute(&self) {
        println!("Update Problems Usecase...");
        let problems = self.api_client.get_problems().await.unwrap();
        for problem in problems {
            println!("{:?}", problem);
        }

        let contests = self.api_client.get_contests().await.unwrap();
        for contest in contests.iter() {
            println!("{:?}", contest.problems);
        }
    }
}
