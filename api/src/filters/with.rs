use warp::{Filter, reject};
use crate::database::{DbConnection, DbPool};

pub fn with_db_connection(pool: DbPool) -> impl Filter<Extract = (DbConnection,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: DbPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DbConnection::new(conn)),
                Err(_) => Err(reject::reject())
            }
        })
}
