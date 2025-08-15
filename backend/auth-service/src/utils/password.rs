use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::Salt};

/// Hashes a password using Argon2 with the provided salt
///
/// # Arguments
/// * `password` - Plain text password to hash
/// * `salt_str` - Salt string to use in hashing
///
/// # Returns
/// Returns the hashed password as a string
///
/// # Panics
/// Panics if:
/// * Salt string is invalid
/// * Password hashing fails
pub fn hash_password(password: &str, salt_str: &str) -> String {
    let salt: Salt = salt_str.try_into().unwrap();

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), salt).unwrap();

    hash.to_string()
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
