use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

mod models;

pub use models::{Token, TokenState};

use super::{response_error, Client};

pub struct AuthClient {
    client: Client,
}

impl AuthClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }

    pub async fn valid_token(&self, token: &String) -> anyhow::Result<()> {
        let response = self
            .client
            .get("/v1/auth/token")
            .header(AUTHORIZATION, token)
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => {
                /* let TokenState { enable } = response.json::<TokenState>().await?;

                Ok(enable) */
                Ok(())
            }
            Err(err) => Err(response_error(
                err,
                "POST",
                "/v1/auth/token",
                response.text().await?,
            )),
        }
    }

    pub async fn refresh_token(&self, token: &String) -> anyhow::Result<String> {
        let request = self
            .client
            .post("/v1/auth/token")
            .header(AUTHORIZATION, token)
            .header(CONTENT_TYPE, "application/json")
            .body("{\"type\": \"refresh\"}");

        let response = request.send().await?;

        match response.error_for_status_ref() {
            Ok(_) => {
                let Token { token: new_token } = response.json().await?;

                Ok(new_token)
            }
            Err(err) => Err(response_error(
                err,
                "POST",
                "/v1/auth/token",
                response.text().await?,
            )),
        }
    }
}
