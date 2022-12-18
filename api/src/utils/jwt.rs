use serde_json::Value;
use crate::config::Config;
use crate::database::models::UserDTO;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

pub fn create_access_token(payload: &Value, config: &Config) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.jwt_secret.as_bytes())?;

    payload.sign_with_key(&key)
}

pub fn create_refresh_token(payload: &Value, config: &Config) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.jwt_secret.as_bytes())?;

    payload.sign_with_key(&key)
}

pub fn verify_token(token: &str, config: &Config) -> Result<(), &'static str> {
    Ok(())
}
