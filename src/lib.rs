pub mod auth;
pub mod book;
pub mod file;

mod client;

pub use client::Client;

pub use auth::AuthClient;
pub use book::BookClient;
pub use file::FileClient;

pub fn response_error(
    error: reqwest::Error,
    method: &str,
    url: &str,
    data: String,
) -> anyhow::Error {
    anyhow::Error::msg(format!(
        "{} {}\n{}\n{}",
        method,
        url,
        error.status().unwrap(),
        data
    ))
}
