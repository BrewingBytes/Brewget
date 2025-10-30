use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use shared_types::TranslationKey;

/// Hashes a password using Argon2 with the provided salt
///
/// # Arguments
/// * `password` - Plain text password to hash
/// * `salt_str` - Salt string to use in hashing
///
/// # Returns
/// * `Ok(String) - The password hashed`
/// * `Err(()) - If the hashing fails`
pub fn hash_password(password: &str) -> Result<String, ()> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ())?;

    Ok(hash.to_string())
}

/// Verifies a password against a hash using Argon2
///
/// # Arguments
/// * `password` - Plain text password to verify
/// * `hash` - Hash string to verify against
///
/// # Returns
/// * `Ok(())` - If password matches hash
/// * `Err(())` - If password doesn't match or verification fails
pub fn verify_password(password: &str, hash: &str) -> Result<(), ()> {
    let password_hash = PasswordHash::new(hash).map_err(|_| ())?;

    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|_| ())
}

/// Validates a password with some basic rules
///
/// # Arguments
/// * `password` - Plain text password to validate
///
/// # Returns
/// * `Ok(())` - If the password is valid
/// * `Err(TranslationKey)` - If the password is not valid with a translation key
pub fn validate_password(password: &str) -> Result<(), TranslationKey> {
    if password.len() < 8 {
        return Err(TranslationKey::PasswordTooShort);
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(TranslationKey::PasswordNoUppercase);
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(TranslationKey::PasswordNoLowercase);
    }

    if !password.chars().any(|c| c.is_numeric()) {
        return Err(TranslationKey::PasswordNoDigit);
    }

    Ok(())
}

/// Checks if a password matches any of the provided password hashes
///
/// # Arguments
/// * `password` - Plain text password to check
/// * `password_hashes` - List of password hashes to check against
///
/// # Returns
/// * `true` - If the password matches any of the hashes
/// * `false` - If the password doesn't match any of the hashes
///
/// # Note
/// This function treats hash verification failures as non-matches (fail-open).
/// While this could theoretically allow password reuse if a hash is corrupted,
/// it prevents users from being locked out. Hash corruption should be extremely
/// rare with Argon2, and the database has integrity constraints to prevent
/// corruption. This is a pragmatic balance between security and availability.
pub fn is_password_in_history(password: &str, password_hashes: &[String]) -> bool {
    password_hashes.iter().fold(false, |acc, hash| {
        acc | verify_password(password, hash).is_ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        let password = "TestPassword123";
        let result = hash_password(password);
        assert!(result.is_ok());

        let hash = result.unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_generates_different_hashes() {
        let password = "TestPassword123";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_success() {
        let password = "TestPassword123";
        let hash = hash_password(password).unwrap();

        let result = verify_password(password, &hash);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_password_wrong_password() {
        let password = "TestPassword123";
        let wrong_password = "WrongPassword456";
        let hash = hash_password(password).unwrap();

        let result = verify_password(wrong_password, &hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let password = "TestPassword123";
        let invalid_hash = "not_a_valid_hash";

        let result = verify_password(password, invalid_hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_password_success() {
        let password = "ValidPass123";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_password_too_short() {
        let password = "Short1";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TranslationKey::PasswordTooShort);
    }

    #[test]
    fn test_validate_password_no_uppercase() {
        let password = "lowercase123";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TranslationKey::PasswordNoUppercase);
    }

    #[test]
    fn test_validate_password_no_number() {
        let password = "NoNumbersHere";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TranslationKey::PasswordNoDigit);
    }

    #[test]
    fn test_validate_password_only_lowercase_and_number() {
        let password = "lowercase1";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TranslationKey::PasswordNoUppercase);
    }

    #[test]
    fn test_validate_password_minimum_valid() {
        let password = "Minimum1";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_password_in_history_empty_list() {
        let password = "TestPassword123";
        let hashes = vec![];
        assert!(!is_password_in_history(password, &hashes));
    }

    #[test]
    fn test_is_password_in_history_match_found() {
        let password = "TestPassword123";
        let hash = hash_password(password).unwrap();
        let hashes = vec![hash];
        assert!(is_password_in_history(password, &hashes));
    }

    #[test]
    fn test_is_password_in_history_no_match() {
        let password1 = "TestPassword123";
        let password2 = "DifferentPassword456";
        let hash = hash_password(password1).unwrap();
        let hashes = vec![hash];
        assert!(!is_password_in_history(password2, &hashes));
    }

    #[test]
    fn test_is_password_in_history_multiple_hashes() {
        let password1 = "TestPassword123";
        let password2 = "TestPassword456";
        let password3 = "TestPassword789";
        let hash1 = hash_password(password1).unwrap();
        let hash2 = hash_password(password2).unwrap();
        let hash3 = hash_password(password3).unwrap();
        let hashes = vec![hash1, hash2, hash3];

        // Test matching each password
        assert!(is_password_in_history(password1, &hashes));
        assert!(is_password_in_history(password2, &hashes));
        assert!(is_password_in_history(password3, &hashes));

        // Test non-matching password
        assert!(!is_password_in_history("NonMatchingPassword1", &hashes));
    }
}
