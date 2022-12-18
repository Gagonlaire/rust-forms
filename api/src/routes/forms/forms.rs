use jsonschema::JSONSchema;
use serde_json::Value;
use warp::{Filter, Reply};
use crate::filters::{with_json_body, with_json_schema};

pub fn register() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let get_form = warp::path!("forms" / String)
        .and(warp::get())
        .and_then(get_form);
    let create_form = warp::path!("forms")
        .and(warp::post())
        .and(with_json_schema(None))
        .and_then(create_form);
    let update_form = warp::path!("forms" / String)
        .and(warp::put())
        .and(with_json_schema(None))
        .and_then(update_form);
    let delete_form = warp::path!("forms" / String)
        .and(warp::delete())
        .and_then(delete_form);

    get_form
        .or(create_form)
        .or(update_form)
        .or(delete_form)
}

async fn get_form(form_id: String) -> Result<impl Reply, warp::Rejection> {
    Ok(format!("get form {}", form_id))
}

async fn create_form(schema: JSONSchema) -> Result<impl Reply, warp::Rejection> {
    println!("{:?}", schema);
    Ok("create form")
}

async fn delete_form(form_id: String) -> Result<impl Reply, warp::Rejection> {
    Ok("delete form")
}

async fn update_form(form_id: String, schema: JSONSchema) -> Result<impl Reply, warp::Rejection> {
    Ok("update form")
}
