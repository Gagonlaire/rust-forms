use argon2::hash_encoded;
use crate::config::Config;
use diesel::serialize::IsNull::No;
use serde_json::json;
use warp::{Filter, Rejection, Reply};
use crate::database::{DbConnection, DbPool};
use crate::database::models::UserDTO;
use crate::filters::{with_config, with_db_connection, with_json_body};
use crate::models::json::{LoginUserSchema, RegisterUserSchema};
use crate::rejections::ApiReject;
use crate::replies::ApiReply;
use crate::utils::argon2::{hash_password, verify_password};
use crate::utils::jwt::{create_access_token, create_refresh_token};

pub fn register(config: Config, db_pool: DbPool) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    let login = warp::path!("login")
        .and(warp::post())
        .and(with_json_body::<LoginUserSchema>(None))
        .and(with_config(config.clone()))
        .and(with_db_connection(db_pool.clone()))
        .and_then(login_handler);

    let register = warp::path!("register")
        .and(warp::post())
        .and(with_json_body::<RegisterUserSchema>(None))
        .and(with_config(config))
        .and(with_db_connection(db_pool))
        .and_then(register_handler);

    login.or(register)
}

async fn login_handler(
    user: LoginUserSchema,
    config: Config,
    mut conn: DbConnection,
) -> Result<impl Reply, Rejection> {
    let user_dto = match conn.get_user_by_email(&user.email) {
        Ok(user) => user,
        Err(error) => return Err(warp::reject::custom(error)),
    };

    if !verify_password(&user.password, &user_dto.password) {
        return Err(
            warp::reject::custom(
                ApiReject::unauthorized("Invalid credentials", None)
            )
        );
    }
    let jwt_payload = json!({
        "id": user_dto.id,
    });
    let access_token = create_access_token(&jwt_payload, &config);
    let refresh_token = create_refresh_token(&jwt_payload, &config);

    if access_token.is_err() || refresh_token.is_err() {
        return Err(warp::reject::custom(
            ApiReject::internal_error()
        ));
    }
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({
            "code": 200,
            "message": "Login successful",
            "access_token": access_token.unwrap(),
            "refresh_token": refresh_token.unwrap(),
            "success": true,
        })),
        warp::http::StatusCode::OK,
    ))
}

async fn register_handler(
    mut new_user: RegisterUserSchema,
    config: Config,
    mut conn: DbConnection,
) -> Result<impl Reply, Rejection> {
    match hash_password(&new_user.password, &config) {
        Some(hashed_password) => new_user.password = hashed_password,
        None => return Err(warp::reject::custom(ApiReject::internal_error()))
    }
    match conn.register_user(new_user) {
        Ok(_) => Ok(ApiReply::created("User successfully created")),
        Err(error) => Err(warp::reject::custom(error)),
    }
}
