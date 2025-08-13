use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaim {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
