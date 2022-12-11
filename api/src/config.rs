use std::net::{IpAddr, SocketAddr};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::utils::{get_env_var, get_env_var_with_default};

#[derive(Debug)]
pub struct Config {
    pub host: SocketAddr,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: SocketAddr::from((
                get_env_var_with_default("HOST", IpAddr::from([127, 0, 0, 1])),
                get_env_var_with_default("PORT", 3000)
            )),
            database_url: get_env_var("DATABASE_URL", "DATABASE_URL must be set"),
        }
    }
}
