use crate::infrastracture::api::yosupo_online_judge::api_client::IYosupoOnlineJudgeAPIClient;

pub struct UpdateYosupoOnlineJudgeProblemUsecase<T>
where
    T: IYosupoOnlineJudgeAPIClient,
{
    api_client: T,
}

impl<T: IYosupoOnlineJudgeAPIClient> UpdateYosupoOnlineJudgeProblemUsecase<T> {
    pub fn new(api_client: T) -> Self {
        Self { api_client }
    }

    pub async fn execute(&self) {
        println!("Update YOJ Problems Usecase");
        // let problems = self.api_client.get_problems().await;
        // for problem in problems.iter() {
        //     println!("{:?}", problem);
        // }

        let contests = self.api_client.get_contests().await;
        for contest in contests.iter() {
            println!("{:?}", contest);
        }

        println!("Finish Update YOJ Problems Usecase");
    }
}
