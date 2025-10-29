use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

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
/// * `Err(String)` - If the password is not valid and an error message
pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".into());
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter".into());
    }

    if !password.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_creates_valid_hash() {
        let password = "TestPassword123";
        let hash_result = hash_password(password);
        
        assert!(hash_result.is_ok());
        let hash = hash_result.unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_password_correct_password() {
        let password = "TestPassword123";
        let hash = hash_password(password).unwrap();
        
        let verify_result = verify_password(password, &hash);
        assert!(verify_result.is_ok());
    }

    #[test]
    fn test_verify_password_incorrect_password() {
        let password = "TestPassword123";
        let wrong_password = "WrongPassword456";
        let hash = hash_password(password).unwrap();
        
        let verify_result = verify_password(wrong_password, &hash);
        assert!(verify_result.is_err());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let password = "TestPassword123";
        let invalid_hash = "invalid_hash_string";
        
        let verify_result = verify_password(password, invalid_hash);
        assert!(verify_result.is_err());
    }

    #[test]
    fn test_validate_password_valid() {
        let password = "ValidPass123";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_password_too_short() {
        let password = "Short1";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password must be at least 8 characters long");
    }

    #[test]
    fn test_validate_password_no_uppercase() {
        let password = "lowercase123";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password must contain at least one uppercase letter");
    }

    #[test]
    fn test_validate_password_no_number() {
        let password = "NoNumberPass";
        let result = validate_password(password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password must contain at least one number");
    }

    #[test]
    fn test_validate_password_minimum_valid() {
        let password = "Abcdefg1";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_hash_password_different_salts() {
        let password = "TestPassword123";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Same password should produce different hashes due to random salts
        assert_ne!(hash1, hash2);
        
        // But both should verify successfully
        assert!(verify_password(password, &hash1).is_ok());
        assert!(verify_password(password, &hash2).is_ok());
    }
}
