use serde::Serialize;

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}

impl TokenResponse {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
