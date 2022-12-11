use diesel::{PgConnection, r2d2, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::database;

pub mod actions;
pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

pub fn establish_connection(database_url: impl Into<String>) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

impl DbConnection {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self {
            connection,
        }
    }

    pub fn get_users(&mut self) -> Result<Vec<models::User>, diesel::result::Error> {
        use database::schema::schema::users::dsl::*;

        let results = users.load::<models::User>(&mut self.connection)?;

        Ok(results)
    }
}
