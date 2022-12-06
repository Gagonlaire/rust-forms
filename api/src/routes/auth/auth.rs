use std::collections::HashMap;
use warp::{Filter, Reply};

fn login_handler(body: HashMap<String, String>) -> impl Reply {
    warp::reply::with_status("login", warp::http::StatusCode::OK)
}

fn register_handler(body: HashMap<String, String>) -> impl Reply {
    warp::reply::with_status("register", warp::http::StatusCode::OK)
}

pub fn get_auth_routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let login = warp::path!("login")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(login_handler);

    let register = warp::path!("register")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(register_handler);

    login.or(register)
}
