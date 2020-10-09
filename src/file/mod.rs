use anyhow;

use super::Client;

pub struct FileClient {
    client: Client,
}

impl FileClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn upload(&self, filepath: &str) -> anyhow::Result<()> {
        let response = self
            .client
            .post(format!("/v1/{}", filepath).as_str())
            .header("Authorization", self.client.token_manager.token())
            .send().await?;

        if response.status().is_success() {
            return Ok(());

        }   else {
            Err(anyhow::Error::msg(format!("Can't upload {}\n{}", response.status().to_string(), response.text().await?)))
        }
    }
}
