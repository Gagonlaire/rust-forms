use diesel::{PgConnection, QueryResult, r2d2};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::{Error, DatabaseErrorKind};
use log::error;

pub mod actions;
pub mod schema;

pub enum DbResult<T> {
    Ok(T),
    NotFound,
    UniqueViolation,
    Unknown,
}

impl<T> From<Error> for DbResult<T> {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => DbResult::NotFound,
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) =>
                DbResult::UniqueViolation,
            _ => {
                error!("Database error: {}", value);
                DbResult::Unknown
            }
        }
    }
}

impl<T> From<QueryResult<T>> for DbResult<T> {
    fn from(result: QueryResult<T>) -> Self {
        match result {
            Ok(value) => DbResult::Ok(value),
            Err(error) => DbResult::from(error)
        }
    }
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
