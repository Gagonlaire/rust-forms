use std::convert::Infallible;
use log::error;
use serde::Serialize;
use warp::{Rejection, Reply};
use crate::rejections::ApiReject;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>,
    success: bool
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    let mut errors = None;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "not Found";
    } else if let Some(error) = err.find::<ApiReject>() {
        message = error.message.as_str();
        code = error.code;
        errors = error.errors.clone();
    } else if let Some(error) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "invalid Body";
        errors = Some(vec![error.to_string()]);
    }  else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "method Not Allowed";
    } else {
        error!("unhandled rejection: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "internal Server Error";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: u16::from(code),
        message: message.into(),
        success: false,
        errors
    });

    Ok(warp::reply::with_status(json, code))
}
