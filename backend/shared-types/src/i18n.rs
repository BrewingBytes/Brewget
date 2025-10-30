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
    CaptchaFailed,
    LinkExpired,
    PasswordTooShort,
    PasswordNoUppercase,
    PasswordNoLowercase,
    PasswordNoDigit,
    PasswordNoSpecialChar,
    PasswordInHistory,
    PasswordChanged,
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
        match lang {
            Language::English => self.translate_en(),
            Language::Spanish => self.translate_es(),
            Language::French => self.translate_fr(),
            Language::German => self.translate_de(),
            Language::Romanian => self.translate_ro(),
        }
    }

    fn translate_en(&self) -> &str {
        match self {
            TranslationKey::UsernamePasswordInvalid => "Username or password is invalid.",
            TranslationKey::EmailNotVerified => {
                "Email has not been verified, please check your inbox."
            }
            TranslationKey::AccountNotActive => {
                "This account has been deactivated. Please contact support."
            }
            TranslationKey::CaptchaFailed => "Captcha verification failed.",
            TranslationKey::LinkExpired => "Link is expired.",
            TranslationKey::PasswordTooShort => "Password must be at least 8 characters long.",
            TranslationKey::PasswordNoUppercase => {
                "Password must contain at least one uppercase letter."
            }
            TranslationKey::PasswordNoLowercase => {
                "Password must contain at least one lowercase letter."
            }
            TranslationKey::PasswordNoDigit => "Password must contain at least one digit.",
            TranslationKey::PasswordNoSpecialChar => {
                "Password must contain at least one special character."
            }
            TranslationKey::PasswordInHistory => {
                "Password cannot be the same as any of your recently used passwords."
            }
            TranslationKey::PasswordChanged => "Password sucessfully changed.",
            TranslationKey::EmailSendFailed => "Failed to send email.",
            TranslationKey::SettingsLoadFailed => "Failed to load settings.",
            TranslationKey::SettingsUpdateFailed => "Failed to update settings.",
            TranslationKey::InternalError => "Something went wrong, please try again!",
            TranslationKey::DatabaseError => "Database error.",
        }
    }

    fn translate_es(&self) -> &str {
        match self {
            TranslationKey::UsernamePasswordInvalid => {
                "El nombre de usuario o la contraseña no son válidos."
            }
            TranslationKey::EmailNotVerified => {
                "El correo electrónico no ha sido verificado, por favor revise su bandeja de entrada."
            }
            TranslationKey::AccountNotActive => {
                "Esta cuenta ha sido desactivada. Por favor contacte al soporte."
            }
            TranslationKey::CaptchaFailed => "La verificación del captcha falló.",
            TranslationKey::LinkExpired => "El enlace ha expirado.",
            TranslationKey::PasswordTooShort => "La contraseña debe tener al menos 8 caracteres.",
            TranslationKey::PasswordNoUppercase => {
                "La contraseña debe contener al menos una letra mayúscula."
            }
            TranslationKey::PasswordNoLowercase => {
                "La contraseña debe contener al menos una letra minúscula."
            }
            TranslationKey::PasswordNoDigit => "La contraseña debe contener al menos un dígito.",
            TranslationKey::PasswordNoSpecialChar => {
                "La contraseña debe contener al menos un carácter especial."
            }
            TranslationKey::PasswordInHistory => {
                "La contraseña no puede ser la misma que ninguna de sus contraseñas utilizadas recientemente."
            }
            TranslationKey::PasswordChanged => "Contraseña cambiada exitosamente.",
            TranslationKey::EmailSendFailed => "Error al enviar el correo electrónico.",
            TranslationKey::SettingsLoadFailed => "Error al cargar la configuración.",
            TranslationKey::SettingsUpdateFailed => "Error al actualizar la configuración.",
            TranslationKey::InternalError => "Algo salió mal, por favor intente de nuevo!",
            TranslationKey::DatabaseError => "Error de base de datos.",
        }
    }

    fn translate_fr(&self) -> &str {
        match self {
            TranslationKey::UsernamePasswordInvalid => {
                "Le nom d'utilisateur ou le mot de passe est invalide."
            }
            TranslationKey::EmailNotVerified => {
                "L'e-mail n'a pas été vérifié, veuillez vérifier votre boîte de réception."
            }
            TranslationKey::AccountNotActive => {
                "Ce compte a été désactivé. Veuillez contacter le support."
            }
            TranslationKey::CaptchaFailed => "La vérification du captcha a échoué.",
            TranslationKey::LinkExpired => "Le lien a expiré.",
            TranslationKey::PasswordTooShort => {
                "Le mot de passe doit contenir au moins 8 caractères."
            }
            TranslationKey::PasswordNoUppercase => {
                "Le mot de passe doit contenir au moins une lettre majuscule."
            }
            TranslationKey::PasswordNoLowercase => {
                "Le mot de passe doit contenir au moins une lettre minuscule."
            }
            TranslationKey::PasswordNoDigit => "Le mot de passe doit contenir au moins un chiffre.",
            TranslationKey::PasswordNoSpecialChar => {
                "Le mot de passe doit contenir au moins un caractère spécial."
            }
            TranslationKey::PasswordInHistory => {
                "Le mot de passe ne peut pas être le même que l'un de vos mots de passe récemment utilisés."
            }
            TranslationKey::PasswordChanged => "Mot de passe changé avec succès.",
            TranslationKey::EmailSendFailed => "Échec de l'envoi de l'e-mail.",
            TranslationKey::SettingsLoadFailed => "Échec du chargement des paramètres.",
            TranslationKey::SettingsUpdateFailed => "Échec de la mise à jour des paramètres.",
            TranslationKey::InternalError => "Quelque chose s'est mal passé, veuillez réessayer!",
            TranslationKey::DatabaseError => "Erreur de base de données.",
        }
    }

    fn translate_de(&self) -> &str {
        match self {
            TranslationKey::UsernamePasswordInvalid => "Benutzername oder Passwort ist ungültig.",
            TranslationKey::EmailNotVerified => {
                "E-Mail wurde nicht verifiziert, bitte überprüfen Sie Ihren Posteingang."
            }
            TranslationKey::AccountNotActive => {
                "Dieses Konto wurde deaktiviert. Bitte kontaktieren Sie den Support."
            }
            TranslationKey::CaptchaFailed => "Captcha-Verifizierung fehlgeschlagen.",
            TranslationKey::LinkExpired => "Der Link ist abgelaufen.",
            TranslationKey::PasswordTooShort => "Das Passwort muss mindestens 8 Zeichen lang sein.",
            TranslationKey::PasswordNoUppercase => {
                "Das Passwort muss mindestens einen Großbuchstaben enthalten."
            }
            TranslationKey::PasswordNoLowercase => {
                "Das Passwort muss mindestens einen Kleinbuchstaben enthalten."
            }
            TranslationKey::PasswordNoDigit => {
                "Das Passwort muss mindestens eine Ziffer enthalten."
            }
            TranslationKey::PasswordNoSpecialChar => {
                "Das Passwort muss mindestens ein Sonderzeichen enthalten."
            }
            TranslationKey::PasswordInHistory => {
                "Das Passwort darf nicht mit einem Ihrer kürzlich verwendeten Passwörter übereinstimmen."
            }
            TranslationKey::PasswordChanged => "Passwort erfolgreich geändert.",
            TranslationKey::EmailSendFailed => "E-Mail konnte nicht gesendet werden.",
            TranslationKey::SettingsLoadFailed => "Einstellungen konnten nicht geladen werden.",
            TranslationKey::SettingsUpdateFailed => {
                "Einstellungen konnten nicht aktualisiert werden."
            }
            TranslationKey::InternalError => {
                "Etwas ist schief gelaufen, bitte versuchen Sie es erneut!"
            }
            TranslationKey::DatabaseError => "Datenbankfehler.",
        }
    }

    fn translate_ro(&self) -> &str {
        match self {
            TranslationKey::UsernamePasswordInvalid => {
                "Numele de utilizator sau parola sunt invalide."
            }
            TranslationKey::EmailNotVerified => {
                "E-mailul nu a fost verificat, vă rugăm să verificați căsuța de e-mail."
            }
            TranslationKey::AccountNotActive => {
                "Acest cont a fost dezactivat. Vă rugăm să contactați suportul."
            }
            TranslationKey::CaptchaFailed => "Verificarea captcha a eșuat.",
            TranslationKey::LinkExpired => "Linkul a expirat.",
            TranslationKey::PasswordTooShort => "Parola trebuie să aibă cel puțin 8 caractere.",
            TranslationKey::PasswordNoUppercase => {
                "Parola trebuie să conțină cel puțin o literă mare."
            }
            TranslationKey::PasswordNoLowercase => {
                "Parola trebuie să conțină cel puțin o literă mică."
            }
            TranslationKey::PasswordNoDigit => "Parola trebuie să conțină cel puțin o cifră.",
            TranslationKey::PasswordNoSpecialChar => {
                "Parola trebuie să conțină cel puțin un caracter special."
            }
            TranslationKey::PasswordInHistory => {
                "Parola nu poate fi aceeași cu niciuna dintre parolele folosite recent."
            }
            TranslationKey::PasswordChanged => "Parola a fost schimbată cu succes.",
            TranslationKey::EmailSendFailed => "Nu s-a putut trimite e-mailul.",
            TranslationKey::SettingsLoadFailed => "Nu s-au putut încărca setările.",
            TranslationKey::SettingsUpdateFailed => "Nu s-au putut actualiza setările.",
            TranslationKey::InternalError => "Ceva a mers prost, vă rugăm să încercați din nou!",
            TranslationKey::DatabaseError => "Eroare de bază de date.",
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
}
