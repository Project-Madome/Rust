use reqwest;

use super::TokenManager;

pub struct Client {
    base_url: String,
    request: reqwest::Client,

    pub token_manager: TokenManager,
}

impl Client {
    pub fn new(base_url: &str, token_manager: TokenManager) -> Self {
        Self {
            base_url: base_url.to_string(),
            token_manager,
            request: reqwest::Client::builder().build().unwrap(),
        }
    }

    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.request
            .get(format!("{}/{}", self.base_url, path).as_str())
    }

    pub fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.request
            .post(format!("{}/{}", self.base_url, path).as_str())
    }

    pub fn delete(&self, path: &str) -> reqwest::RequestBuilder {
        self.request
            .delete(format!("{}/{}", self.base_url, path).as_str())
    }

    pub fn patch(&self, path: &str) -> reqwest::RequestBuilder {
        self.request
            .patch(format!("{}/{}", self.base_url, path).as_str())
    }

    pub fn put(&self, path: &str) -> reqwest::RequestBuilder {
        self.request
            .put(format!("{}/{}", self.base_url, path).as_str())
    }

    pub fn head(&self, path: &str) -> reqwest::RequestBuilder {
        self.request
            .head(format!("{}/{}", self.base_url, path).as_str())
    }
}
