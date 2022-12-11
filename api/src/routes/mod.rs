#[path = "auth/auth.rs"]
mod auth;
#[path = "collector/collector.rs"]
mod collector;

use warp::{Filter};
use crate::config::{Config};
use crate::database::{DbConnection, DbPool};
use crate::filters::with_db_connection;

pub fn build(_: Config, db_manager: DbPool) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let collector = collector::get_collector();
    let auth_routes = auth::get_auth_routes();

    let test = warp::path("test")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db_connection(db_manager))
        .map(|mut e: DbConnection| {
            println!("{:?}", e.get_users());
            "ok"
        });

    collector
        .or(auth_routes)
        .or(test)
}
