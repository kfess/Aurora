use std::sync::Arc;

use super::{
    aoj::api_client::AojAPIClientTrait, atcoder::api_client::AtcoderAPIClientTrait,
    cf::api_client::CFAPIClientTrait, yoj::api_client::YOJAPIClientTrait,
    yuki::api_client::YukicoderAPIClientTrait,
};

pub struct APIClientFactory {
    atcoder_client: Option<Arc<dyn AtcoderAPIClientTrait>>,
    cf_client: Option<Arc<dyn CFAPIClientTrait>>,
    yuki_client: Option<Arc<dyn YukicoderAPIClientTrait>>,
    aoj_client: Option<Arc<dyn AojAPIClientTrait>>,
    yoj_client: Option<Arc<dyn YOJAPIClientTrait>>,
}

impl APIClientFactory {
    pub fn new() -> Self {
        Self {
            atcoder_client: None,
            cf_client: None,
            yuki_client: None,
            aoj_client: None,
            yoj_client: None,
        }
    }

    pub fn with_atcoder_client(mut self) -> Self {
        self.atcoder_client = Some(Arc::new(super::atcoder::api_client::AtcoderAPIClient::new()));
        self
    }

    pub fn get_atcoder_client(&self) -> Result<Arc<dyn AtcoderAPIClientTrait>, &'static str> {
        self.atcoder_client
            .clone()
            .ok_or("AtCoder client is not set")
    }

    pub fn with_cf_client(mut self) -> Self {
        self.cf_client = Some(Arc::new(super::cf::api_client::CFAPIClient::new()));
        self
    }

    pub fn get_cf_client(&self) -> Result<Arc<dyn CFAPIClientTrait>, &'static str> {
        self.cf_client.clone().ok_or("Codeforces client is not set")
    }

    pub fn with_yuki_client(mut self) -> Self {
        self.yuki_client = Some(Arc::new(super::yuki::api_client::YukicoderAPIClient::new()));
        self
    }

    pub fn get_yuki_client(&self) -> Result<Arc<dyn YukicoderAPIClientTrait>, &'static str> {
        self.yuki_client
            .clone()
            .ok_or("Yukicoder client is not set")
    }

    pub fn with_aoj_client(mut self) -> Self {
        self.aoj_client = Some(Arc::new(super::aoj::api_client::AojAPIClient::new()));
        self
    }

    pub fn get_aoj_client(&self) -> Result<Arc<dyn AojAPIClientTrait>, &'static str> {
        self.aoj_client.clone().ok_or("AOJ client is not set")
    }

    pub fn with_yoj_client(mut self) -> Self {
        self.yoj_client = Some(Arc::new(super::yoj::api_client::YOJAPIClient::new()));
        self
    }

    pub fn get_yoj_client(&self) -> Result<Arc<dyn YOJAPIClientTrait>, &'static str> {
        self.yoj_client.clone().ok_or("YOJ client is not set")
    }
}
