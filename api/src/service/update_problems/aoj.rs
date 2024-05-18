use crate::infra::api::aoj::api_client::AojAPIClientTrait;

pub struct UpdateAojUsecase<T>
where
    T: AojAPIClientTrait,
{
    api_client: T,
}

impl<T> UpdateAojUsecase<T>
where
    T: AojAPIClientTrait,
{
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
