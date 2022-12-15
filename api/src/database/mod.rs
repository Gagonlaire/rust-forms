use diesel::{PgConnection, r2d2};
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub mod actions;
pub mod schema;
pub mod models {
    pub use crate::models::database::*;
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_pool_connection(database_url: impl Into<String>) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct DbConnection {
    pub connection: DbConn,
}

impl DbConnection {
    pub fn new(connection: DbConn) -> Self {
        Self {
            connection,
        }
    }
}
