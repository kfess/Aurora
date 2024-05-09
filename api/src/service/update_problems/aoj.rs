use crate::infra::api::aoj::api_client::IAojAPIClient;

pub struct UpdateAojUsecase<T>
where
    T: IAojAPIClient,
{
    api_client: T,
}

impl<T> UpdateAojUsecase<T>
where
    T: IAojAPIClient,
{
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
