use crate::infrastracture::api::codeforces::api_client::ICodeforcesAPICLient;

pub struct UpdateCodeforcesProblemUsecase<T>
where
    T: ICodeforcesAPICLient,
{
    api_client: T,
}

impl<T: ICodeforcesAPICLient> UpdateCodeforcesProblemUsecase<T> {
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
