#[path = "auth/auth.rs"]
mod auth;
#[path = "collector/collector.rs"]
mod collector;

use warp::{Filter};
use crate::config::AppData;

pub fn build(app_data: &AppData) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let collector = collector::get_collector(app_data);
    let auth_routes = auth::get_auth_routes(app_data);

    collector
        .or(auth_routes)
}
