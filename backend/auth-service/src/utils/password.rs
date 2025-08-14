use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::Salt};

pub fn hash_password(password: &str, salt_str: &str) -> String {
    let salt: Salt = salt_str.try_into().unwrap();

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), salt)
        .unwrap()
        .hash
        .unwrap();

    hash.to_string()
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), ()> {
    let password_hash = PasswordHash::new(hash).map_err(|_| ())?;

    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|_| ())
}
