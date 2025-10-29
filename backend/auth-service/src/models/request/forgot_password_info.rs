use serde::Deserialize;

/// Represents the info required for sending a forgot password email
///
/// This struct is used to deserialize JSON data sent to the `/forgot-password` endpoint
///
/// # Fields
/// * `email` - The email of the user account
///
/// # Example
/// ```json
/// {
///     "email": "test@test.com",
/// }
/// ```
#[derive(Deserialize)]
pub struct ForgotPasswordInfo {
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_forgot_password_info_deserialization() {
        let json = r#"{"email":"user@example.com"}"#;
        
        let forgot_info: ForgotPasswordInfo = serde_json::from_str(json).unwrap();
        assert_eq!(forgot_info.email, "user@example.com");
    }

    #[test]
    fn test_forgot_password_info_various_email_formats() {
        let test_cases = vec![
            "simple@example.com",
            "user.name@example.co.uk",
            "user+tag@subdomain.example.com",
            "123@numbers.com",
        ];
        
        for email in test_cases {
            let json = format!(r#"{{"email":"{}"}}"#, email);
            
            let forgot_info: ForgotPasswordInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(forgot_info.email, email);
        }
    }

    #[test]
    fn test_forgot_password_info_empty_email() {
        let json = r#"{"email":""}"#;
        
        let forgot_info: ForgotPasswordInfo = serde_json::from_str(json).unwrap();
        assert_eq!(forgot_info.email, "");
    }
}
