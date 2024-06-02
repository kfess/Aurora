use crate::infra::api::cf::api_client::CFAPIClient;

pub struct UpdateCodeforcesUsecase<C>
where
    C: CFAPIClient,
{
    api_client: C,
}

impl<C: CFAPIClient> UpdateCodeforcesUsecase<C> {
    pub fn new(api_client: C) -> Self {
        return Self { api_client };
    }

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
