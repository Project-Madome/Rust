use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

mod models;

pub use models::{Book, ContentType, Language, Metadata, MetadataBook};

use anyhow;
use bytes::Bytes;

use super::{response_error, Client};

pub struct BookClient {
    client: Client,
}

impl BookClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }

    pub async fn create_book(&self, token: &String, book: Book) -> anyhow::Result<()> {
        let book = serde_json::to_string(&book).unwrap();

        let response = self
            .client
            .post("/v1/book")
            .header(AUTHORIZATION, token)
            .header(CONTENT_TYPE, "application/json")
            .body(Bytes::from(book))
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(err) => Err(response_error(
                err,
                "POST",
                "/v1/book",
                response.text().await?,
            )),
        }
    }

    pub async fn get_image_list(
        &self,
        token: &String,
        book_id: u32,
    ) -> anyhow::Result<Vec<String>> {
        let url = format!("/v1/book/{}/image/list", book_id);

        let response = self
            .client
            .get(url.as_str())
            .header(AUTHORIZATION, token)
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => {
                let image_list = response.json::<Vec<String>>().await?;
                Ok(image_list)
            }
            Err(err) => Err(response_error(
                err,
                "GET",
                url.as_str(),
                response.text().await?,
            )),
        }

        /* Err(anyhow::Error::msg(format!(
            "Can't get image list {}\n{}",
            response.status().to_string(),
            response.text().await?
        ))) */
    }
}
