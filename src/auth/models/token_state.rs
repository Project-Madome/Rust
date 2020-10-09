use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenState {
    pub enable: bool,
    // expires: u64,
}
