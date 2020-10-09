use anyhow;
use bytes::Bytes;

use super::Client;

pub struct FileClient<'a> {
    pub client: &'a Client,
}

impl<'a> FileClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn upload(&self, url_path: &str, buf: Bytes) -> anyhow::Result<()> {
        let response = self
            .client
            .post(format!("/v1/{}", url_path).as_str())
            .header("Authorization", self.client.token_manager.token())
            .body(buf)
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        } else {
            Err(anyhow::Error::msg(format!(
                "Can't upload {}\n{}",
                response.status().to_string(),
                response.text().await?
            )))
        }
    }
}
