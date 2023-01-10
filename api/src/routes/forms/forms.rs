use crate::config::Config;
use warp::{Filter, Reply};
use crate::database::{DbConnection, DbPool, DbResult};
use crate::filters::{with_db_connection, with_jwt_auth, with_form_schema};
use crate::models::json::{FormSchema, JwtPayload};
use crate::rejections::ApiReject;
use crate::replies::ApiReply;

pub fn register(
    config: Config,
    db_pool: DbPool,
) -> impl Filter<Extract=impl Reply, Error=warp::Rejection> + Clone {
    let get_form = warp::path!("forms" / i32)
        .and(warp::get())
        .and(with_jwt_auth(config.clone()))
        .and(with_db_connection(db_pool.clone()))
        .and_then(get_form);
    let create_form = warp::path!("forms")
        .and(warp::post())
        .and(with_jwt_auth(config.clone()))
        .and(with_form_schema(None))
        .and(with_db_connection(db_pool))
        .and_then(create_form);
    let update_form = warp::path!("forms" / String)
        .and(warp::put())
        .and(with_jwt_auth(config.clone()))
        .and(with_form_schema(None))
        .and_then(update_form);
    let delete_form = warp::path!("forms" / String)
        .and(warp::delete())
        .and(with_jwt_auth(config))
        .and_then(delete_form);

    get_form
        .or(create_form)
        .or(update_form)
        .or(delete_form)
}

async fn get_form(
    form_id: i32,
    user: JwtPayload,
    mut conn: DbConnection,
) -> Result<impl Reply, warp::Rejection> {
    match conn.get_form(form_id) {
        DbResult::Ok(form) => {
            if user.id == form.id {
                Ok(warp::reply::json(&form))
            } else {
                Err(warp::reject::custom(
                    ApiReject::unauthorized("Unauthorized", None)
                ))
            }
        }
        _ => Err(warp::reject::custom(ApiReject::internal_error()))
    }
}

async fn create_form(
    user: JwtPayload,
    schema: FormSchema,
    mut conn: DbConnection,
) -> Result<impl Reply, warp::Rejection> {
    match conn.create_form(&schema, user.id) {
        DbResult::Ok(_) => Ok(ApiReply::ok("form created successfully")),
        _ => Err(warp::reject::custom(
            ApiReject::internal_error()
        ))
    }
}

async fn delete_form(_form_id: String, _user: JwtPayload) -> Result<impl Reply, warp::Rejection> {
    Ok("delete form")
}

async fn update_form(
    _form_id: String,
    _user: JwtPayload,
    _schema: FormSchema,
) -> Result<impl Reply, warp::Rejection> {
    Ok("update form")
}
