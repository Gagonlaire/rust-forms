use crate::config::{Config};
use argon2;
use argon2::hash_encoded;

pub fn hash_password(password: impl Into<String>, config: &Config) -> Option<String> {
    match hash_encoded(
        password.into().as_bytes(),
        config.argon2_salt.as_bytes(),
        &config.argon2_config) {
        Ok(hashed_password) => Some(hashed_password),
        Err(_) => None
    }
}

pub fn verify_password(password: impl Into<String>, hashed_password: impl Into<String>) -> bool {
    argon2::verify_encoded(&hashed_password.into(), password.into().as_bytes()).unwrap_or(false)
}
