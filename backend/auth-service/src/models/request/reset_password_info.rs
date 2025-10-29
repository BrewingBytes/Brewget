use serde::Deserialize;
use uuid::Uuid;

/// Represents credentials required for setting a new password
///
/// This struct is used to deserialize JSON data sent to the `/change-password` endpoint
///
/// # Fields
/// * `id` - The id of the forgot password link
/// * `password` - The new user's password for authentication
///
/// # Example
/// ```json
/// {
///     "id": "abcd-efgh-aaaa",
///     "password": "secretpassword123"
/// }
/// ```
#[derive(Deserialize)]
pub struct ResetPasswordInfo {
    pub id: Uuid,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_reset_password_info_deserialization() {
        let uuid = Uuid::new_v4();
        let json = format!(
            r#"{{"id":"{}","password":"NewSecurePass123"}}"#,
            uuid
        );
        
        let reset_info: ResetPasswordInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(reset_info.id, uuid);
        assert_eq!(reset_info.password, "NewSecurePass123");
    }

    #[test]
    fn test_reset_password_info_deserialization_with_spaces() {
        let uuid = Uuid::new_v4();
        let json = format!(
            r#"{{
                "id": "{}",
                "password": "New Password With Spaces 123"
            }}"#,
            uuid
        );
        
        let reset_info: ResetPasswordInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(reset_info.id, uuid);
        assert_eq!(reset_info.password, "New Password With Spaces 123");
    }

    #[test]
    fn test_reset_password_info_invalid_uuid() {
        let json = r#"{"id":"not-a-valid-uuid","password":"Pass123"}"#;
        
        let result: Result<ResetPasswordInfo, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
