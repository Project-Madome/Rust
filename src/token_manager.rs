use std::fs;

pub struct TokenManager {
    token: String,
}

impl TokenManager {
    pub fn new() -> Self {
        let token = fs::read_to_string("./.token").unwrap();

        Self { token }
    }

    pub fn refresh(&mut self, token: String) -> anyhow::Result<()> {
        fs::write("./.token", &token)?;
        self.token = token;
        Ok(())
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }
}
