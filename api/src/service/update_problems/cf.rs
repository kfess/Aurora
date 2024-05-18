use crate::infra::api::cf::api_client::CFAPIClientTrait;

pub struct UpdateCodeforcesUsecase<T>
where
    T: CFAPIClientTrait,
{
    api_client: T,
}

impl<T: CFAPIClientTrait> UpdateCodeforcesUsecase<T> {
    pub fn new(api_client: T) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
