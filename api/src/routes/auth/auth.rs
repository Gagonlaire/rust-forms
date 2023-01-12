use crate::config::Config;
use serde_json::{to_value};
use warp::{Filter, Rejection, Reply};
use crate::database::{DbConnection, DbPool, DbResult};
use crate::filters::{with_config, with_db_connection, with_json_body};
use crate::models::json::{JwtPayload, LoginSuccess, LoginUserSchema, RefreshSuccess, RefreshTokenSchema, RegisterUserSchema};
use crate::rejections::ApiReject;
use crate::replies::ApiReply;
use crate::utils::argon2::{hash_password, verify_password};
use crate::utils::jwt::{create_tokens, Jwt, verify_token, create_token};

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
        .and(with_config(config.clone()))
        .and(with_db_connection(db_pool))
        .and_then(register_handler);
    let refresh = warp::path!("refresh")
        .and(warp::post())
        .and(with_json_body::<RefreshTokenSchema>(None))
        .and(with_config(config))
        .and_then(refresh_handler);

    warp::path!("auth" / ..)
        .and(login.or(register).or(refresh))
}

async fn login_handler(
    user: LoginUserSchema,
    config: Config,
    mut conn: DbConnection,
) -> Result<impl Reply, Rejection> {
    let user_dto = match conn.get_user_by_email(&user.email) {
        DbResult::Ok(user) => user,
        DbResult::NotFound => return Err(warp::reject::custom(
            ApiReject::unauthorized("invalid credentials", None)
        )),
        _ => return Err(warp::reject::custom(ApiReject::internal_error()))
    };

    if !verify_password(&user.password, user_dto.password) {
        return Err(
            warp::reject::custom(
                ApiReject::unauthorized("invalid credentials", None)
            )
        );
    }
    let payload = JwtPayload {
        id: user_dto.id,
        admin: user_dto.admin,
    };
    let tokens = create_tokens(&config, &to_value(payload).unwrap());

    match tokens {
        None => Err(warp::reject::custom(ApiReject::internal_error())),
        Some(tokens) => {
            Ok(warp::reply::json(&LoginSuccess {
                code: 200,
                access_token: tokens.0,
                refresh_token: tokens.1,
                message: "Login successful",
                success: true,
            }))
        }
    }
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
        DbResult::Ok(_) => Ok(ApiReply::created("User successfully created")),
        DbResult::UniqueViolation => Err(warp::reject::custom(ApiReject::bad_request("Email already taken", None))),
        _ => Err(warp::reject::custom(ApiReject::internal_error()))
    }
}

async fn refresh_handler(
    refresh_token: RefreshTokenSchema,
    config: Config,
) -> Result<impl Reply, Rejection>  {
    let payload = match verify_token(&refresh_token.refresh_token, &config) {
        Jwt::Valid(payload) => payload,
        error => return Err(warp::reject::custom(
            ApiReject::unauthorized(error.to_string(), None)
        ))
    };

    let access_token = match create_token(&config, &to_value(payload).unwrap(), config.jwt_access_expiration) {
        Ok(token) => token,
        Err(_) => return Err(warp::reject::custom(ApiReject::internal_error()))
    };

    Ok(warp::reply::json(&RefreshSuccess {
        code: 200,
        access_token,
        message: "Refresh successful",
        success: true,
    }))
}
