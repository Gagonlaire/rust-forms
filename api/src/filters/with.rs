use std::convert::Infallible;
use jsonschema::JSONSchema;
use warp::{Filter, reject};
use crate::database::{DbConnection, DbPool};
use serde::de::DeserializeOwned;
use serde_json::Value;
use warp::body::json;
use crate::config::Config;
use crate::rejections::ApiReject;
use crate::database::DbConn;

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

pub fn with_json_schema(
    limit: Option<u64>
) -> impl Filter<Extract=(JSONSchema, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(limit.unwrap_or(DEFAULT_MAX_JSON_SIZE))
        .and(json())
        .and_then(|schema: Value| async move {
            match JSONSchema::compile(&schema) {
                Ok(schema) => Ok(schema),
                Err(_) => Err(reject::reject())
            }
        })
}
