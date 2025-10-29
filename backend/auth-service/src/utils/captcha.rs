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
