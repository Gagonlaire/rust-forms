use diesel::{PgConnection, r2d2};

pub mod actions;
pub mod models;

pub fn create_connection_pool(database_url: impl Into<String>) -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url.into());

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
