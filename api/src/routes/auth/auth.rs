use std::sync::Arc;
use diesel::RunQueryDsl;
use warp::{Filter, Reply};
use crate::config::AppData;
use crate::models::json::{LoginSchema, NewUser};
use crate::filters::app_data_filter;

fn login_handler(app_data: Arc<AppData>, body: LoginSchema) -> impl Reply {
    warp::reply::with_status("login", warp::http::StatusCode::OK)
}

fn register_handler(app_data: Arc<AppData>, body: NewUser) -> impl Reply {
    use crate::models::schema::users::dsl::*;

    let mut conn = app_data.pool.get().unwrap();

    diesel::insert_into(users)
        .values(&body)
        .execute(&mut conn)
        .expect("Error saving new user");
    warp::reply::with_status("register", warp::http::StatusCode::OK)
}

pub fn get_auth_routes(app_data: &Arc<AppData>) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let login = warp::path!("login")
        .and(warp::post())
        .and(app_data_filter(app_data.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json::<LoginSchema>())
        .map(login_handler);

    let register = warp::path!("register")
        .and(warp::post())
        .and(app_data_filter(app_data.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json::<NewUser>())
        .map(register_handler);

    login.or(register)
}
