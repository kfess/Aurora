use crate::infra::api::codeforces::api_client::ICodeforcesAPICLient;

pub struct UpdateCodeforcesUsecase<T>
where
    T: ICodeforcesAPICLient,
{
    api_client: T,
}

impl<T: ICodeforcesAPICLient> UpdateCodeforcesUsecase<T> {
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
