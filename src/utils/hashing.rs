use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::password_hash::{Error as PasswordError, PasswordHash, PasswordVerifier};
use argon2::PasswordHasher;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let hash_password = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(hash_password.to_string())
}

pub fn verify_password(db_password: &str, credential_password: &str) -> Result<(), PasswordError> {
    let db_password_hash = PasswordHash::new(db_password)?;
    let argon = argon2::Argon2::default();
    argon.verify_password(credential_password.as_bytes(), &db_password_hash)?;
    Ok(())
}
