use serde_json::Value;
use crate::config::Config;
use jwt::{SignWithKey, VerifyWithKey};
use chrono::{Utc, Duration};

pub enum Jwt {
    Expired,
    Invalid,
    Valid(i64),
}

impl ToString for Jwt {
    fn to_string(&self) -> String {
        match self {
            Jwt::Expired => "Token expired".to_string(),
            Jwt::Invalid => "Invalid token".to_string(),
            Jwt::Valid(id) => id.to_string(),
        }
    }
}

pub fn create_token(config: &Config, payload: &Value, exp: i64) -> Result<String, jwt::Error> {
    let mut payload = payload.clone();
    let now = Utc::now();

    payload["iat"] = Value::from(now.timestamp());
    payload["exp"] = Value::from(now.timestamp() + Duration::seconds(exp).num_seconds());
    payload.sign_with_key(&config.jwt_secret)
}

// Create an access and a refresh token
pub fn create_tokens(config: &Config, payload: &Value) -> Option<(String, String)> {
    let access_token = create_token(config, payload, config.jwt_access_expiration);
    let refresh_token = create_token(config, payload, config.jwt_refresh_expiration);

    if access_token.is_err() || refresh_token.is_err() {
        return None;
    }
    Some((access_token.unwrap(), refresh_token.unwrap()))
}

pub fn verify_token(token: &str, config: &Config) -> Jwt {
    let payload: Value = match token.verify_with_key(&config.jwt_secret) {
        Ok(payload) => payload,
        Err(_) => return Jwt::Invalid,
    };
    let exp = payload["exp"].as_i64().unwrap_or(0);

    if exp < Utc::now().timestamp() {
        return Jwt::Expired;
    }
    Jwt::Valid(payload["id"].as_i64().unwrap_or(0))
}
