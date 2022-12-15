#[path = "auth/auth.rs"]
mod auth;
#[path = "collector/collector.rs"]
mod collector;

use warp::{Filter};
use crate::config::{Config};
use crate::database::{DbPool};

pub fn build(config: Config, db_pool: DbPool) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let collector = collector::get_collector();
    let auth_routes = auth::get_auth_routes();

    collector
        .or(auth_routes)
}
