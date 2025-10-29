use serde::{Deserialize, Serialize};

/// Request body for Cloudflare Turnstile verification
#[derive(Serialize)]
struct TurnstileVerifyRequest {
    secret: String,
    response: String,
}

/// Response from Cloudflare Turnstile verification endpoint
#[derive(Deserialize)]
struct TurnstileVerifyResponse {
    success: bool,
    #[serde(rename = "error-codes")]
    error_codes: Option<Vec<String>>,
}

/// Verifies a Cloudflare Turnstile captcha token
///
/// Makes an HTTP POST request to Cloudflare's siteverify endpoint to validate
/// the captcha token provided by the client.
///
/// # Arguments
///
/// * `token` - The captcha token from the client
/// * `secret` - The Turnstile secret key from configuration
///
/// # Returns
///
/// * `Ok(())` - Token is valid
/// * `Err(String)` - Token is invalid with error message
///
/// # Example
///
/// ```rust,no_run
/// use auth_service::utils::captcha::verify_turnstile;
///
/// let result = verify_turnstile("token", "secret").await;
/// assert!(result.is_ok());
/// ```
pub async fn verify_turnstile(token: &str, secret: &str) -> Result<(), String> {
    let client = reqwest::Client::new();

    let request_body = TurnstileVerifyRequest {
        secret: secret.to_string(),
        response: token.to_string(),
    };

    let response = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to verify captcha: {}", e))?;

    let verify_response: TurnstileVerifyResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse captcha response: {}", e))?;

    if verify_response.success {
        Ok(())
    } else {
        let error_msg = verify_response
            .error_codes
            .map(|codes| codes.join(", "))
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(format!("Captcha verification failed: {}", error_msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These are unit tests for the data structures and error handling logic.
    // Integration tests with actual Cloudflare API would require mocking or test tokens.

    #[test]
    fn test_turnstile_request_serialization() {
        let request = TurnstileVerifyRequest {
            secret: "test_secret".to_string(),
            response: "test_token".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("test_secret"));
        assert!(serialized.contains("test_token"));
        assert!(serialized.contains("secret"));
        assert!(serialized.contains("response"));
    }

    #[test]
    fn test_turnstile_response_deserialization_success() {
        let json = r#"{"success": true}"#;
        let response: TurnstileVerifyResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert!(response.error_codes.is_none());
    }

    #[test]
    fn test_turnstile_response_deserialization_with_errors() {
        let json = r#"{"success": false, "error-codes": ["invalid-input-response", "timeout-or-duplicate"]}"#;
        let response: TurnstileVerifyResponse = serde_json::from_str(json).unwrap();
        assert!(!response.success);
        assert!(response.error_codes.is_some());
        
        let errors = response.error_codes.unwrap();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0], "invalid-input-response");
        assert_eq!(errors[1], "timeout-or-duplicate");
    }

    #[test]
    fn test_turnstile_response_deserialization_no_error_codes() {
        let json = r#"{"success": false}"#;
        let response: TurnstileVerifyResponse = serde_json::from_str(json).unwrap();
        assert!(!response.success);
        assert!(response.error_codes.is_none());
    }
}
