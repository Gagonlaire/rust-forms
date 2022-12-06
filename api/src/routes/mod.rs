use std::collections::HashMap;
use warp::{Filter};

#[path = "auth/auth.rs"]
mod auth;
#[path = "collector/collector.rs"]
mod collector;

pub fn build() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let collector = collector::get_collector();
    let auth_routes = auth::get_auth_routes();

    warp::get()
        .and(
            warp::path("api")
                .map(|| {"ok"})
        )
}
