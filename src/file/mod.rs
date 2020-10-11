use std::io::Write;

use anyhow;
use bytes::Bytes;

use super::{response_error, Client};

static BOUNDARY: &'static str = "----MadomeBoundary583r2ae3h3w3hs470e";

fn formdata(buf: Bytes, content_type: String) -> std::io::Result<Vec<u8>> {
    let mut data = vec![];

    write!(data, "--{}\r\n", BOUNDARY)?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"file\"; filename=\"a.img\"\r\n"
    )?;
    write!(data, "Content-Type: {}\r\n", content_type)?;
    write!(data, "\r\n")?;

    let mut data = data.into_iter().chain(buf.into_iter()).collect::<Vec<_>>();

    write!(data, "\r\n")?;
    write!(data, "--{}--\r\n", BOUNDARY)?;

    Ok(data)
}

pub struct FileClient {
    client: Client,
}

impl FileClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }

    /// ## Aurguments
    /// - url_path: don't add slash to front
    pub async fn upload(&self, token: &String, url_path: &str, buf: Bytes) -> anyhow::Result<()> {
        let url = format!("/v1/{}", url_path);

        // let buf_len = buf.len();
        let content_type = mime_guess::from_path(url_path).first().unwrap().to_string();
        let formdata_buf = formdata(buf, content_type)?;

        let request = self
            .client
            .post(url.as_str())
            .header("Authorization", token)
            .header(
                "Content-Type",
                format!("multipart/form-data; boundary={}", BOUNDARY),
            )
            // .header("Content-Length", formdata_buf.len())
            .body(formdata_buf);

        let response = request.send().await?;

        match response.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(err) => Err(response_error(
                err,
                "POST",
                url.as_str(),
                response.text().await?,
            )),
        }
    }
}
