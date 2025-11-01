use serde::Serialize;

/// Translation keys for frontend localization
///
/// Each variant represents a specific message that the frontend should translate
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TranslationKey {
    /// Password successfully changed message
    PasswordSuccessfullyChanged,
    /// Generic OK message
    Ok,
    /// Account created message
    AccountCreated,
    /// Forgot password link sent message
    ForgotPasswordLinkSent,
    /// Account verified message
    AccountVerified,
    /// User does not exist error
    UserDoesNotExist,
    /// Link is expired error
    LinkIsExpired,
    /// Password cannot be reused error
    PasswordCannotBeReused,
    /// Something went wrong generic error
    SomethingWentWrong,
    /// Captcha verification failed error
    CaptchaVerificationFailed,
    /// Username or password is invalid error
    UsernameOrPasswordInvalid,
    /// Email has not been verified error
    EmailNotVerified,
    /// Account has been deleted temporarily error
    AccountDeletedTemporarily,
    /// Username length too short error
    UsernameTooShort,
    /// Email address is not valid error
    EmailAddressInvalid,
    /// Username or email already used error
    UsernameOrEmailAlreadyUsed,
    /// Could not create account error
    CouldNotCreateAccount,
    /// You are not logged in error
    NotLoggedIn,
    /// Token has expired error
    TokenExpired,
    /// Token is invalid error
    TokenInvalid,
    /// Internal server error
    InternalServerError,
    /// Password validation error - generic
    PasswordValidationError,
    /// Password must be at least 8 characters long
    PasswordTooShort,
    /// Password must contain at least one uppercase letter
    PasswordMissingUppercase,
    /// Password must contain at least one number
    PasswordMissingNumber,
    /// Username not found error
    UsernameNotFound,
    /// Username or email not found error
    UsernameOrEmailNotFound,
    /// Could not verify account error
    CouldNotVerifyAccount,
    /// Could not update password error
    CouldNotUpdatePassword,
    /// Activation link not found error
    ActivationLinkNotFound,
    /// Forgot password link not found error
    ForgotPasswordLinkNotFound,
    /// Failed to retrieve password history error
    FailedToRetrievePasswordHistory,
}

/// A message response structure containing a translation key
///
/// This struct is used to tell the frontend which translation key to use
///
/// # Fields
/// * `translation_key` - The translation key indicating which message to display
///
/// # Example
/// ```json
/// {
///     "translation_key": "ACCOUNT_CREATED"
/// }
/// ```
#[derive(Serialize, Clone, Debug)]
pub struct TranslationKeyMessage {
    pub translation_key: TranslationKey,
}

/// The response for the /health route
///
/// # Fields
/// * `status` - The current status of the service
/// * `database` - The current status of the connection to the db (optional)
/// * `version` - The current version of the service
///
/// # Example
/// ```json
/// {
///     "status": "healthy",
///     "database": "connected",
///     "version": "0.0.2"
/// }
/// ```
#[derive(Serialize)]
pub struct Health {
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseConnection>,
    pub version: String,
}

/// The enum for the Health Status
#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

/// The enum for the Database Connection Status
#[derive(Serialize)]
pub enum DatabaseConnection {
    Connected,
    Disconnected,
}

/// A JWT response structure
///
/// This struct is used to serialize response JWT into JSON format
///
/// # Fields
/// * `token` - The JWT to be sent in the response after login
///
/// # Example
/// ```json
/// {
///     "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
/// }
/// ```
#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_key_serialization() {
        let test_cases = vec![
            (TranslationKey::AccountCreated, "ACCOUNT_CREATED"),
            (
                TranslationKey::PasswordSuccessfullyChanged,
                "PASSWORD_SUCCESSFULLY_CHANGED",
            ),
            (TranslationKey::Ok, "OK"),
            (
                TranslationKey::ForgotPasswordLinkSent,
                "FORGOT_PASSWORD_LINK_SENT",
            ),
            (TranslationKey::AccountVerified, "ACCOUNT_VERIFIED"),
        ];

        for (key, expected) in test_cases {
            let msg = TranslationKeyMessage {
                translation_key: key,
            };
            let json = serde_json::to_string(&msg).unwrap();
            assert!(
                json.contains(expected),
                "Expected '{}' to contain '{}'",
                json,
                expected
            );
        }
    }

    #[test]
    fn test_translation_key_message_format() {
        let msg = TranslationKeyMessage {
            translation_key: TranslationKey::AccountCreated,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"translation_key":"ACCOUNT_CREATED"}"#);
    }
}
