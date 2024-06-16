use std::sync::Arc;

#[derive(Clone)]
pub struct ApiClient {
    pub client: Arc<reqwest::Client>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
        }
    }
}
