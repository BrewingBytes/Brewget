/// Simple internationalization support for error messages
///
/// Provides translation capabilities for common error messages based on language code.
/// Supports: English (en), Spanish (es), French (fr), German (de)

/// Gets a translated error message based on the message key and language
///
/// # Arguments
/// * `key` - The message key to translate
/// * `lang` - The language code (e.g., "en", "es", "fr", "de")
///
/// # Returns
/// The translated message string, or English as fallback if translation not found
pub fn translate(key: &str, lang: &str) -> String {
    match lang {
        "es" => translate_es(key),
        "fr" => translate_fr(key),
        "de" => translate_de(key),
        _ => translate_en(key),
    }
}

/// Extract language from Accept-Language header
///
/// # Arguments
/// * `accept_language` - The Accept-Language header value
///
/// # Returns
/// The language code or "en" as default
pub fn extract_language(accept_language: Option<&str>) -> String {
    accept_language
        .and_then(|header| header.split(',').next())
        .and_then(|lang| lang.split('-').next())
        .map(|lang| lang.trim().to_lowercase())
        .filter(|lang| matches!(lang.as_str(), "en" | "es" | "fr" | "de"))
        .unwrap_or_else(|| "en".to_string())
}

fn translate_en(key: &str) -> String {
    match key {
        "auth.username_or_password_invalid" => "Username or password is invalid.",
        "auth.email_not_verified" => "Email has not been verified, please check your inbox.",
        "auth.account_deleted" => "Account has been deleted temporarily",
        "auth.user_not_exist" => "User does not exist.",
        "auth.link_expired" => "Link is expired.",
        "auth.username_length" => "Username length cannot be less or equal to 3 characters.",
        "auth.email_invalid" => "Email address is not valid.",
        "auth.username_email_used" => "Username or email is already used.",
        "auth.account_created" => "Account has been created.",
        _ => key,
    }
    .to_string()
}

fn translate_es(key: &str) -> String {
    match key {
        "auth.username_or_password_invalid" => "El usuario o la contraseña no son válidos.",
        "auth.email_not_verified" => {
            "El correo electrónico no ha sido verificado, por favor revisa tu bandeja de entrada."
        }
        "auth.account_deleted" => "La cuenta ha sido eliminada temporalmente",
        "auth.user_not_exist" => "El usuario no existe.",
        "auth.link_expired" => "El enlace ha expirado.",
        "auth.username_length" => {
            "La longitud del nombre de usuario no puede ser menor o igual a 3 caracteres."
        }
        "auth.email_invalid" => "La dirección de correo electrónico no es válida.",
        "auth.username_email_used" => "El nombre de usuario o correo electrónico ya está en uso.",
        "auth.account_created" => "La cuenta ha sido creada.",
        _ => return translate_en(key),
    }
    .to_string()
}

fn translate_fr(key: &str) -> String {
    match key {
        "auth.username_or_password_invalid" => {
            "Le nom d'utilisateur ou le mot de passe n'est pas valide."
        }
        "auth.email_not_verified" => {
            "L'email n'a pas été vérifié, veuillez vérifier votre boîte de réception."
        }
        "auth.account_deleted" => "Le compte a été supprimé temporairement",
        "auth.user_not_exist" => "L'utilisateur n'existe pas.",
        "auth.link_expired" => "Le lien a expiré.",
        "auth.username_length" => {
            "La longueur du nom d'utilisateur ne peut pas être inférieure ou égale à 3 caractères."
        }
        "auth.email_invalid" => "L'adresse email n'est pas valide.",
        "auth.username_email_used" => {
            "Le nom d'utilisateur ou l'email est déjà utilisé."
        }
        "auth.account_created" => "Le compte a été créé.",
        _ => return translate_en(key),
    }
    .to_string()
}

fn translate_de(key: &str) -> String {
    match key {
        "auth.username_or_password_invalid" => "Benutzername oder Passwort ist ungültig.",
        "auth.email_not_verified" => {
            "Die E-Mail wurde nicht verifiziert, bitte überprüfen Sie Ihr Postfach."
        }
        "auth.account_deleted" => "Das Konto wurde vorübergehend gelöscht",
        "auth.user_not_exist" => "Der Benutzer existiert nicht.",
        "auth.link_expired" => "Der Link ist abgelaufen.",
        "auth.username_length" => {
            "Die Länge des Benutzernamens darf nicht kleiner oder gleich 3 Zeichen sein."
        }
        "auth.email_invalid" => "Die E-Mail-Adresse ist ungültig.",
        "auth.username_email_used" => {
            "Der Benutzername oder die E-Mail wird bereits verwendet."
        }
        "auth.account_created" => "Das Konto wurde erstellt.",
        _ => return translate_en(key),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_english() {
        assert_eq!(
            translate("auth.username_or_password_invalid", "en"),
            "Username or password is invalid."
        );
    }

    #[test]
    fn test_translate_spanish() {
        assert_eq!(
            translate("auth.username_or_password_invalid", "es"),
            "El usuario o la contraseña no son válidos."
        );
    }

    #[test]
    fn test_extract_language() {
        assert_eq!(extract_language(Some("en-US,en;q=0.9")), "en");
        assert_eq!(extract_language(Some("es-ES")), "es");
        assert_eq!(extract_language(Some("fr")), "fr");
        assert_eq!(extract_language(Some("de-DE")), "de");
        assert_eq!(extract_language(Some("ja-JP")), "en"); // Fallback to English
        assert_eq!(extract_language(None), "en"); // Default to English
    }
}
