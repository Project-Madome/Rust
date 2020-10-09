use std::fs;

pub struct TokenManager {
    token: String,
    path: String,
}

impl TokenManager {
    pub fn new(token_file_path: &str) -> Self {
        let token = fs::read_to_string(token_file_path).unwrap();

        Self { token, path: token_file_path.to_string() }
    }

    pub fn refresh(&mut self, token: String) -> anyhow::Result<()> {
        fs::write(&self.path, &token)?;
        self.token = token;
        Ok(())
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }
}
