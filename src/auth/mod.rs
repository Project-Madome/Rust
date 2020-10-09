mod models;

pub use models::{Token, TokenState};

use super::Client;

pub struct AuthClient {
    client: Client,
}

impl AuthClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn valid_token(&self) -> anyhow::Result<bool> {
        let response = self
            .client
            .get("/v1/auth/token")
            .header("Authorization", self.client.token_manager.token())
            .send()
            .await?;

        let TokenState { enable } = response.json::<TokenState>().await?;

        Ok(enable)
    }

    pub async fn refresh_token(&self) -> anyhow::Result<String> {
        let response = self
            .client
            .post("/v1/auth/token")
            .header("Authorization", self.client.token_manager.token())
            .body("{\"type\": \"refresh\"}")
            .send()
            .await?;

        let Token { token: new_token } = response.json::<Token>().await?;

        Ok(new_token)
    }
}
