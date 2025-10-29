use serde::Deserialize;

/// Represents credentials required for user registration
///
/// This struct is used to deserialize JSON data sent to the `/register` endpoint
///
/// # Fields
/// * `username` - The user's register identifier
/// * `email`    - The user's email
/// * `password` - The user's password for authentication
///
/// # Example
/// ```json
/// {
///     "username": "user",
///     "email": "user@example.com",
///     "password": "secretpassword123"
/// }
/// ```
#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_register_info_deserialization() {
        let json = r#"{
            "username": "newuser",
            "email": "newuser@example.com",
            "password": "SecurePass123"
        }"#;
        
        let register_info: RegisterInfo = serde_json::from_str(json).unwrap();
        assert_eq!(register_info.username, "newuser");
        assert_eq!(register_info.email, "newuser@example.com");
        assert_eq!(register_info.password, "SecurePass123");
    }

    #[test]
    fn test_register_info_deserialization_various_emails() {
        let test_cases = vec![
            ("user@example.com", "user@example.com"),
            ("user.name@example.com", "user.name@example.com"),
            ("user+tag@example.co.uk", "user+tag@example.co.uk"),
        ];
        
        for (email, expected) in test_cases {
            let json = format!(
                r#"{{"username":"testuser","email":"{}","password":"Pass123"}}"#,
                email
            );
            
            let register_info: RegisterInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(register_info.email, expected);
        }
    }

    #[test]
    fn test_register_info_deserialization_compact() {
        let json = r#"{"username":"u","email":"u@e.co","password":"P1"}"#;
        
        let register_info: RegisterInfo = serde_json::from_str(json).unwrap();
        assert_eq!(register_info.username, "u");
        assert_eq!(register_info.email, "u@e.co");
        assert_eq!(register_info.password, "P1");
    }
}
