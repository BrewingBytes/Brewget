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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_token_claim_serialization() {
        let claim = TokenClaim {
            sub: Arc::from("user123"),
            iat: 1692115200,
            exp: 1692118800,
        };
        
        let json = serde_json::to_string(&claim).unwrap();
        assert!(json.contains("user123"));
        assert!(json.contains("1692115200"));
        assert!(json.contains("1692118800"));
    }

    #[test]
    fn test_token_claim_deserialization() {
        let json = r#"{"sub":"user456","iat":1692115200,"exp":1692118800}"#;
        
        let claim: TokenClaim = serde_json::from_str(json).unwrap();
        assert_eq!(&*claim.sub, "user456");
        assert_eq!(claim.iat, 1692115200);
        assert_eq!(claim.exp, 1692118800);
    }

    #[test]
    fn test_token_claim_roundtrip() {
        let original = TokenClaim {
            sub: Arc::from("roundtrip_user"),
            iat: 1000000000,
            exp: 2000000000,
        };
        
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TokenClaim = serde_json::from_str(&json).unwrap();
        
        assert_eq!(&*original.sub, &*deserialized.sub);
        assert_eq!(original.iat, deserialized.iat);
        assert_eq!(original.exp, deserialized.exp);
    }

    #[test]
    fn test_token_claim_expiration_in_future() {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let claim = TokenClaim {
            sub: Arc::from("test_user"),
            iat: now,
            exp: now + 3600, // 1 hour from now
        };
        
        // Expiration should be after issued time
        assert!(claim.exp > claim.iat);
    }

    #[test]
    fn test_token_claim_with_uuid_subject() {
        use uuid::Uuid;
        
        let user_id = Uuid::new_v4();
        let claim = TokenClaim {
            sub: Arc::from(user_id.to_string()),
            iat: 1692115200,
            exp: 1692118800,
        };
        
        // Should be able to parse back to UUID
        let parsed_uuid = Uuid::parse_str(&claim.sub);
        assert!(parsed_uuid.is_ok());
        assert_eq!(parsed_uuid.unwrap(), user_id);
    }
}
