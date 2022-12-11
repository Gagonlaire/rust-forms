use warp::{Filter, Reply};
use crate::models::json::{LoginSchema, NewUser};

fn login_handler(body: LoginSchema) -> impl Reply {
    warp::reply::with_status("login", warp::http::StatusCode::OK)
}

fn register_handler(new_user: NewUser) -> impl Reply {
    println!("Registering user: {:?}", new_user);
    warp::reply::with_status("register", warp::http::StatusCode::OK)
}

pub fn get_auth_routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let login = warp::path!("login")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json::<LoginSchema>())
        .map(login_handler);

    let register = warp::path!("register")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json::<NewUser>())
        .map(register_handler);

    login.or(register)
}
