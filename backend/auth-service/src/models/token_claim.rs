use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaim {
    pub sub: Arc<str>,
    pub iat: usize,
    pub exp: usize,
}
