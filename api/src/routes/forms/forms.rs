use crate::config::Config;
use jsonschema::JSONSchema;
use warp::{Filter, Reply};
use crate::filters::{with_json_schema, with_jwt_auth};

pub fn register(config: Config) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let get_form = warp::path!("forms" / String)
        .and(warp::get())
        .and(with_jwt_auth(config.clone()))
        .and_then(get_form);
    let create_form = warp::path!("forms")
        .and(warp::post())
        .and(with_jwt_auth(config.clone()))
        .and(with_json_schema(None))
        .and_then(create_form);
    let update_form = warp::path!("forms" / String)
        .and(warp::put())
        .and(with_jwt_auth(config.clone()))
        .and(with_json_schema(None))
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

async fn get_form(form_id: String, user_id: u64) -> Result<impl Reply, warp::Rejection> {
    Ok(format!("get form {}", form_id))
}

async fn create_form(user_id: u64, schema: JSONSchema) -> Result<impl Reply, warp::Rejection> {
    println!("{schema:?}");
    Ok("create form")
}

async fn delete_form(_form_id: String, user_id: u64) -> Result<impl Reply, warp::Rejection> {
    Ok("delete form")
}

async fn update_form(_form_id: String, user_id: u64, _schema: JSONSchema) -> Result<impl Reply, warp::Rejection> {
    Ok("update form")
}
