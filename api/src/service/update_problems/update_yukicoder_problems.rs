use crate::infrastracture::api::yukicoder::api_client::IYukicoderAPIClient;
use core::time;
use std::thread;

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
        let problem_ids = self
            .api_client
            .get_problems()
            .await
            .unwrap()
            .iter()
            .map(|problem| problem.problem_id)
            .collect::<Vec<u64>>();

        for problem_id in problem_ids[..3].iter() {
            thread::sleep(time::Duration::from_millis(1000));
            let problem = self.api_client.get_problem(*problem_id).await.unwrap();
        }

        println!("{:?}", problem_ids);
    }
}
