use crate::utils::errors::CustomError;
use argon2::{Argon2, PasswordHasher};
use password_hash::{rand_core::OsRng, SaltString};

pub fn hash_password(password: &str) -> Result<String, CustomError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| CustomError::Internal("Password hashing failed".into()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, CustomError> {
    use argon2::{PasswordHash, PasswordVerifier};

    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| CustomError::Internal("Invalid password hash format".into()))?;

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) =>Ok(true),
        Err(password_hash::Error::Password) => Ok(false), // WRONG PASSWORD
        Err(_) => Err(CustomError::Internal("Password verification failed".into())),
    }
}
