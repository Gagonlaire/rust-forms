use std::convert::Infallible;
use warp::{Filter, reject};
use crate::database::{DbConnection, DbPool};
use serde::de::DeserializeOwned;
use warp::body::json;
use crate::config::Config;
use crate::rejections::ApiReject;
use crate::utils::jwt::{Jwt, verify_token};

static DEFAULT_MAX_JSON_SIZE: u64 = 16 * 1024;

pub fn with_db_connection(
    pool: DbPool
) -> impl Filter<Extract=(DbConnection, ), Error=warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: DbPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DbConnection::new(conn)),
                Err(error) => Err(reject::custom(ApiReject::from_diesel_error(error)))
            }
        })
}

pub fn with_config(
    config: Config
) -> impl Filter<Extract=(Config, ), Error=Infallible> + Clone {
    warp::any().map(move || config.clone())
}

pub fn with_json_body<T: DeserializeOwned + Send>(
    limit: Option<u64>
) -> impl Filter<Extract=(T, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(limit.unwrap_or(DEFAULT_MAX_JSON_SIZE))
        .and(json())
}

pub fn with_jwt_auth(
    config: Config
) -> impl Filter<Extract=(i64, ), Error=warp::Rejection> + Clone {
    warp::header::optional::<String>("Authorization")
        .and(with_config(config))
        .and_then(|token: Option<String>, config: Config| async move {
            if token.is_none() {
                return Err(reject::custom(
                    ApiReject::unauthorized("Missing token", None)
                ));
            }
            let token = token.unwrap();

            match verify_token(token.trim_start_matches("Bearer "), &config) {
                Jwt::Valid(id) => Ok(id),
                error => Err(reject::custom(ApiReject::from(error)))
            }
        })
}
