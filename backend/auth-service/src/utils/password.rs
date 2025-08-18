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
