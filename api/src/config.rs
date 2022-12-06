use std::net::SocketAddr;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Debug)]
pub struct AppData {
    pub config: Config,
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Debug)]
pub struct Config {
    pub host: SocketAddr,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: SocketAddr::from(([127, 0, 0, 1], 8080)),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
