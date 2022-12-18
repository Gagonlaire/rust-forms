use serde_json::Value;
use crate::config::Config;
use crate::database::models::UserDTO;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use chrono::{Utc, Duration};

pub enum JWT {
    Expired,
    Invalid,
    Unknown,
    Valid(u64),
}

impl ToString for JWT {
    fn to_string(&self) -> String {
        match self {
            JWT::Expired => "Token expired".to_string(),
            JWT::Invalid => "Invalid token".to_string(),
            JWT::Unknown => "Unknown".to_string(),
            JWT::Valid(id) => id.to_string(),
        }
    }
}

fn set_payload_timestamp(payload: &mut Value, exp_in: i64) {
    let now = Utc::now();

    payload["iat"] = Value::from(now.timestamp());
    payload["exp"] = Value::from(now.timestamp() + Duration::seconds(exp_in).num_seconds());
}

pub fn create_access_token(payload: &Value, config: &Config) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.jwt_secret.as_bytes()).unwrap();
    let mut payload = payload.clone();

    set_payload_timestamp(&mut payload, config.jwt_access_expiration);
    payload.sign_with_key(&key)
}

pub fn create_refresh_token(payload: &Value, config: &Config) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.jwt_secret.as_bytes()).unwrap();
    let mut payload = payload.clone();

    set_payload_timestamp(&mut payload, config.jwt_refresh_expiration);
    payload.sign_with_key(&key)
}

pub fn verify_token(token: &str, config: &Config) -> JWT {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.jwt_secret.as_bytes()).unwrap();
    let payload: Value = match token.verify_with_key(&key) {
        Ok(payload) => payload,
        Err(_) => return JWT::Invalid,
    };
    let exp = payload["exp"].as_i64().unwrap_or(0);

    if exp < Utc::now().timestamp() {
        return JWT::Expired;
    }
    JWT::Valid(payload["id"].as_u64().unwrap_or(0))
}
