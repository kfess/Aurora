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

    pub fn execute(&self) {
        println!("Update Problems Usecase");
    }
}
