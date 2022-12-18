use warp::{Filter, Rejection, Reply};
use crate::database::{DbConnection, DbPool};
use crate::filters::{with_db_connection, with_json_body};
use crate::models::json::{LoginUserSchema, RegisterUserSchema};

pub fn register(db_pool: DbPool) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let login = warp::path!("login")
        .and(warp::post())
        .and(with_json_body::<LoginUserSchema>(None))
        .and_then(login_handler);

    let register = warp::path!("register")
        .and(warp::post())
        .and(with_json_body::<RegisterUserSchema>(None))
        .and(with_db_connection(db_pool))
        .and_then(register_handler);

    login.or(register)
}

async fn login_handler(body: LoginUserSchema) -> Result<impl Reply, warp::Rejection> {
    println!("Login handler: {:?}", body);
    Ok(warp::reply::with_status("login", warp::http::StatusCode::OK))
}

async fn register_handler(new_user: RegisterUserSchema, mut conn: DbConnection) -> Result<impl Reply, warp::Rejection> {
    match conn.register_user(new_user) {
        Ok(_) => Ok(warp::reply::with_status("register", warp::http::StatusCode::OK)),
        Err(_) => Err(warp::reject::reject())
    }
}
