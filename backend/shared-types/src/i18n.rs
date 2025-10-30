/// Translation keys for all backend messages
/// These are sent to the frontend for client-side translation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslationKey {
    // Auth service
    UsernamePasswordInvalid,
    EmailNotVerified,
    AccountNotActive,
    AccountDeleted,
    CaptchaFailed,
    LinkExpired,
    PasswordTooShort,
    PasswordNoUppercase,
    PasswordNoLowercase,
    PasswordNoDigit,
    PasswordNoSpecialChar,
    PasswordInHistory,
    PasswordChanged,
    NotLoggedIn,
    TokenExpired,
    TokenInvalid,
    UserNotExist,
    AccountVerified,
    UsernameTooShort,
    ForgotPasswordSent,
    // Email service
    EmailSendFailed,
    // Settings service
    SettingsLoadFailed,
    SettingsUpdateFailed,
    // General
    InternalError,
    DatabaseError,
}
