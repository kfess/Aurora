// use crate::infra::api::yuki::api_client::YukicoderAPIClientTrait;

// pub struct UpdateYukicoderUsecase<T>
// where
//     T: YukicoderAPIClientTrait,
// {
//     api_client: T,
// }

// impl<T: YukicoderAPIClientTrait> UpdateYukicoderUsecase<T> {
//     pub fn new(api_client: T) -> Self {
//         return Self { api_client };
//     }

//     pub async fn update_recent(&self) {
//         log::info!("Yukicoder: update recent problems and contests");

//         let problems = self.api_client.get_problems(true).await.unwrap();
//         for problem in problems {
//             println!("{:?}", problem);
//         }

//         let contests = self.api_client.get_contests(true).await.unwrap();
//         for contest in contests.iter() {
//             println!("{:?}", contest);
//         }
//     }

//     pub async fn update_all(&self) {
//         log::info!("Yukicoder: update all problems and contests");

//         let problems = self.api_client.get_problems(false).await.unwrap();
//         for problem in problems {
//             println!("{:?}", problem);
//         }

//         let contests = self.api_client.get_contests(false).await.unwrap();
//         for contest in contests.iter() {
//             println!("{:?}", contest);
//         }
//     }
// }

use crate::infra::api::factory::APIClientFactory;

pub struct UpdateYukicoderUsecase {
    api_client_factory: APIClientFactory,
}

impl UpdateYukicoderUsecase {
    pub fn new(api_client_factory: APIClientFactory) -> Self {
        return Self { api_client_factory };
    }

    pub async fn execute(&self) {
        log::info!("Yukicoder: update problems and contests");

        if let Ok(yuki_client) = self.api_client_factory.get_yuki_client() {
            let problems = yuki_client.get_problems(true).await.unwrap();
            for problem in problems {
                println!("{:?}", problem);
            }
        } else {
            // エラーハンドリング
            log::error!("Yukicoder client is not set");
        }
    }
}
