use serde::Deserialize;

/// Represents credentials required for user authentication
///
/// This struct is used to deserialize JSON data sent to the `/login` endpoint
///
/// # Fields
/// * `username` - The user's login identifier
/// * `password` - The user's password for authentication
///
/// # Example
/// ```json
/// {
///     "username": "user",
///     "password": "secretpassword123"
/// }
/// ```
#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_login_info_deserialization() {
        let json = r#"{"username":"testuser","password":"TestPass123"}"#;
        
        let login_info: LoginInfo = serde_json::from_str(json).unwrap();
        assert_eq!(login_info.username, "testuser");
        assert_eq!(login_info.password, "TestPass123");
    }

    #[test]
    fn test_login_info_deserialization_with_spaces() {
        let json = r#"{
            "username": "user with spaces",
            "password": "Pass with spaces 123"
        }"#;
        
        let login_info: LoginInfo = serde_json::from_str(json).unwrap();
        assert_eq!(login_info.username, "user with spaces");
        assert_eq!(login_info.password, "Pass with spaces 123");
    }

    #[test]
    fn test_login_info_deserialization_special_chars() {
        let json = r#"{"username":"user@domain.com","password":"P@ssw0rd!#$"}"#;
        
        let login_info: LoginInfo = serde_json::from_str(json).unwrap();
        assert_eq!(login_info.username, "user@domain.com");
        assert_eq!(login_info.password, "P@ssw0rd!#$");
    }
}
