use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Translation messages loaded from JSON files
#[derive(Debug, Deserialize, Clone)]
pub struct Messages {
    username_password_invalid: String,
    email_not_verified: String,
    account_not_active: String,
    account_deleted: String,
    captcha_failed: String,
    link_expired: String,
    password_too_short: String,
    password_no_uppercase: String,
    password_no_lowercase: String,
    password_no_digit: String,
    password_no_special_char: String,
    password_in_history: String,
    password_changed: String,
    email_send_failed: String,
    settings_load_failed: String,
    settings_update_failed: String,
    internal_error: String,
    database_error: String,
    not_logged_in: String,
    token_expired: String,
    token_invalid: String,
    user_not_exist: String,
    account_verified: String,
    username_too_short: String,
    forgot_password_sent: String,
}

/// Load all translations at compile time
static TRANSLATIONS: Lazy<HashMap<&'static str, Messages>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Load English translations - must succeed as it's the fallback
    let en_json = include_str!("../locales/en.json");
    let en_messages = serde_json::from_str::<Messages>(en_json)
        .expect("Failed to parse English translations - this is a critical error");
    map.insert("en", en_messages);

    // Load Spanish translations
    let es_json = include_str!("../locales/es.json");
    let es_messages =
        serde_json::from_str::<Messages>(es_json).expect("Failed to parse Spanish translations");
    map.insert("es", es_messages);

    // Load French translations
    let fr_json = include_str!("../locales/fr.json");
    let fr_messages =
        serde_json::from_str::<Messages>(fr_json).expect("Failed to parse French translations");
    map.insert("fr", fr_messages);

    // Load German translations
    let de_json = include_str!("../locales/de.json");
    let de_messages =
        serde_json::from_str::<Messages>(de_json).expect("Failed to parse German translations");
    map.insert("de", de_messages);

    // Load Romanian translations
    let ro_json = include_str!("../locales/ro.json");
    let ro_messages =
        serde_json::from_str::<Messages>(ro_json).expect("Failed to parse Romanian translations");
    map.insert("ro", ro_messages);

    map
});

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Spanish,
    French,
    German,
    Romanian,
}

impl Language {
    /// Parse language from string code
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "es" => Language::Spanish,
            "fr" => Language::French,
            "de" => Language::German,
            "ro" => Language::Romanian,
            _ => Language::English, // Default to English
        }
    }

    /// Get language code
    pub fn to_code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::German => "de",
            Language::Romanian => "ro",
        }
    }
}

/// Translation keys for all backend messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl TranslationKey {
    /// Get translated message for the given language
    pub fn translate(&self, lang: Language) -> &str {
        let code = lang.to_code();
        let messages = TRANSLATIONS
            .get(code)
            .or_else(|| TRANSLATIONS.get("en"))
            .expect("English translations must always be available");

        match self {
            TranslationKey::UsernamePasswordInvalid => &messages.username_password_invalid,
            TranslationKey::EmailNotVerified => &messages.email_not_verified,
            TranslationKey::AccountNotActive => &messages.account_not_active,
            TranslationKey::AccountDeleted => &messages.account_deleted,
            TranslationKey::CaptchaFailed => &messages.captcha_failed,
            TranslationKey::LinkExpired => &messages.link_expired,
            TranslationKey::PasswordTooShort => &messages.password_too_short,
            TranslationKey::PasswordNoUppercase => &messages.password_no_uppercase,
            TranslationKey::PasswordNoLowercase => &messages.password_no_lowercase,
            TranslationKey::PasswordNoDigit => &messages.password_no_digit,
            TranslationKey::PasswordNoSpecialChar => &messages.password_no_special_char,
            TranslationKey::PasswordInHistory => &messages.password_in_history,
            TranslationKey::PasswordChanged => &messages.password_changed,
            TranslationKey::NotLoggedIn => &messages.not_logged_in,
            TranslationKey::TokenExpired => &messages.token_expired,
            TranslationKey::TokenInvalid => &messages.token_invalid,
            TranslationKey::UserNotExist => &messages.user_not_exist,
            TranslationKey::AccountVerified => &messages.account_verified,
            TranslationKey::UsernameTooShort => &messages.username_too_short,
            TranslationKey::ForgotPasswordSent => &messages.forgot_password_sent,
            TranslationKey::EmailSendFailed => &messages.email_send_failed,
            TranslationKey::SettingsLoadFailed => &messages.settings_load_failed,
            TranslationKey::SettingsUpdateFailed => &messages.settings_update_failed,
            TranslationKey::InternalError => &messages.internal_error,
            TranslationKey::DatabaseError => &messages.database_error,
        }
    }
}

/// Translator struct for getting messages
pub struct Translator {
    language: Language,
}

impl Translator {
    /// Create a new translator for the given language
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Create a translator from language code
    pub fn from_code(code: &str) -> Self {
        Self::new(Language::from_code(code))
    }

    /// Get translated message
    pub fn translate(&self, key: TranslationKey) -> String {
        key.translate(self.language).to_string()
    }

    /// Get language
    pub fn language(&self) -> Language {
        self.language
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("en"), Language::English);
        assert_eq!(Language::from_code("es"), Language::Spanish);
        assert_eq!(Language::from_code("fr"), Language::French);
        assert_eq!(Language::from_code("de"), Language::German);
        assert_eq!(Language::from_code("ro"), Language::Romanian);
        assert_eq!(Language::from_code("unknown"), Language::English);
    }

    #[test]
    fn test_translation_en() {
        let translator = Translator::new(Language::English);
        assert_eq!(
            translator.translate(TranslationKey::UsernamePasswordInvalid),
            "Username or password is invalid."
        );
    }

    #[test]
    fn test_translation_es() {
        let translator = Translator::new(Language::Spanish);
        assert_eq!(
            translator.translate(TranslationKey::UsernamePasswordInvalid),
            "El nombre de usuario o la contraseña no son válidos."
        );
    }

    #[test]
    fn test_translation_ro() {
        let translator = Translator::new(Language::Romanian);
        assert_eq!(
            translator.translate(TranslationKey::UsernamePasswordInvalid),
            "Numele de utilizator sau parola sunt invalide."
        );
    }

    #[test]
    fn test_translator_from_code() {
        let translator = Translator::from_code("ro");
        assert_eq!(translator.language(), Language::Romanian);
    }

    #[test]
    fn test_translations_loaded() {
        // Verify all languages are loaded
        assert!(TRANSLATIONS.contains_key("en"));
        assert!(TRANSLATIONS.contains_key("es"));
        assert!(TRANSLATIONS.contains_key("fr"));
        assert!(TRANSLATIONS.contains_key("de"));
        assert!(TRANSLATIONS.contains_key("ro"));
    }

    #[test]
    fn test_password_changed_spelling() {
        let translator = Translator::new(Language::English);
        let message = translator.translate(TranslationKey::PasswordChanged);
        // Verify correct spelling (not "sucessfully")
        assert!(message.contains("successfully"));
        assert_eq!(message, "Password successfully changed.");
    }
}
