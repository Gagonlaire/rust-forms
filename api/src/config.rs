use std::net::{IpAddr, SocketAddr};
use hmac::digest::KeyInit;
use hmac::Hmac;
use sha2::Sha256;
use crate::utils::env::{get_env_var, get_env_var_with_default};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: SocketAddr,
    pub database_url: String,
    pub argon2_config: argon2::Config<'static>,
    pub argon2_salt: String,
    pub jwt_secret: Hmac<Sha256>,
    pub jwt_access_expiration: i64,
    pub jwt_refresh_expiration: i64,
}

impl Default for Config {
    fn default() -> Self {
        let jwt_secret = get_env_var("JWT_SECRET", "JWT_SECRET must be set");
        let jwt_secret = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

        Self {
            host: SocketAddr::from((
                get_env_var_with_default("HOST", IpAddr::from([127, 0, 0, 1])),
                get_env_var_with_default("PORT", 3000)
            )),
            database_url: get_env_var("DATABASE_URL", "DATABASE_URL must be set"),
            argon2_config: argon2::Config::default(),
            argon2_salt: get_env_var("ARGON2_SALT", "ARGON2_SALT must be set"),
            jwt_secret,
            jwt_access_expiration: get_env_var_with_default("JWT_ACCESS_EXPIRATION", 3600),
            jwt_refresh_expiration: get_env_var_with_default("JWT_REFRESH_EXPIRATION", 86400),
        }
    }
}
