use crate::infrastracture::api::yosupo_online_judge::api_client::IYosupoOnlineJudgeAPIClient;

pub struct UpdateYosupoOnlineJudgeProblemUsecase<T>
where
    T: IYosupoOnlineJudgeAPIClient,
{
    api_client: T,
}

impl<T: IYosupoOnlineJudgeAPIClient> UpdateYosupoOnlineJudgeProblemUsecase<T> {
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
