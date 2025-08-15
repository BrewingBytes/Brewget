use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// JWT claims structure used for token generation and validation
///
/// Represents the standard claims included in the JWT payload
///
/// # Fields
/// * `sub` - Subject claim, typically contains user identifier
/// * `iat` - Issued At timestamp (in seconds since Unix epoch)
/// * `exp` - Expiration timestamp (in seconds since Unix epoch)
///
/// # Example
/// ```json
/// {
///     "sub": "user123",
///     "iat": 1692115200,
///     "exp": 1692118800
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct TokenClaim {
    pub sub: Arc<str>,
    pub iat: usize,
    pub exp: usize,
}
