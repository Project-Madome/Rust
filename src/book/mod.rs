mod models;
pub use models::{Book, ContentType, Language, Metadata, MetadataBook};

use anyhow;
use bytes::Bytes;

use super::Client;

pub struct BookClient<'a> {
    pub client: &'a Client,
}

impl<'a> BookClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create_book(&self, book: Book) -> anyhow::Result<()> {
        let book = serde_json::to_string(&book).unwrap();

        let response = self
            .client
            .post("/v1/book")
            .header("Authorization", self.client.token_manager.token())
            .body(Bytes::from(book))
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        } else {
            Err(anyhow::Error::msg(format!(
                "Can't create book {}\n{}",
                response.status().to_string(),
                response.text().await?
            )))
        }
    }

    pub async fn get_image_list(&self, book_id: u32) -> anyhow::Result<Vec<String>> {
        let response = self
            .client
            .get(format!("/v1/book/{}/image/list", book_id).as_str())
            .header("Authorization", self.client.token_manager.token())
            .send()
            .await?;

        if response.status().is_success() {
            let image_list = response.json::<Vec<String>>().await?;
            return Ok(image_list);
        } else {
            Err(anyhow::Error::msg(format!(
                "Can't get image list {}\n{}",
                response.status().to_string(),
                response.text().await?
            )))
        }
    }
}
