use crate::config::Config;
use warp::{Filter, Reply};
use crate::database::{DbConnection, DbPool, DbResult};
use crate::filters::{with_db_connection, with_jwt_auth, with_form_schema};
use crate::models::json::FormSchema;
use crate::rejections::ApiReject;
use crate::replies::ApiReply;

pub fn register(
    config: Config,
    db_pool: DbPool,
) -> impl Filter<Extract=impl Reply, Error=warp::Rejection> + Clone {
    let get_form = warp::path!("forms" / String)
        .and(warp::get())
        .and(with_jwt_auth(config.clone()))
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

async fn get_form(form_id: String, _user_id: i64) -> Result<impl Reply, warp::Rejection> {
    Ok(format!("get form {form_id}"))
}

async fn create_form(
    user_id: i64,
    schema: FormSchema,
    mut conn: DbConnection,
) -> Result<impl Reply, warp::Rejection> {
    match conn.create_form(&schema, user_id as i32) {
        DbResult::Ok(_) => Ok(ApiReply::ok("form created successfully")),
        _ => Err(warp::reject::custom(
            ApiReject::internal_error()
        ))
    }
}

async fn delete_form(_form_id: String, _user_id: i64) -> Result<impl Reply, warp::Rejection> {
    Ok("delete form")
}

async fn update_form(
    _form_id: String,
    _user_id: i64,
    _schema: FormSchema,
) -> Result<impl Reply, warp::Rejection> {
    Ok("update form")
}
