use std::fmt::{Debug};
use log::error;
use warp::http::StatusCode;
use warp::reject::Reject;
use std::error::Error;
use crate::utils::jwt::Jwt;

#[derive(Debug)]
pub struct ApiReject {
    pub message: String,
    pub code: StatusCode,
    pub errors: Option<Vec<String>>,
}

impl ApiReject {
    pub fn new(message: impl Into<String>, code: StatusCode, errors: Option<Vec<String>>) -> Self {
        Self {
            message: message.into(),
            code,
            errors,
        }
    }

    pub fn bad_request(message: impl Into<String>, errors: Option<Vec<String>>) -> Self {
        Self::new(message, StatusCode::BAD_REQUEST, errors)
    }

    pub fn unauthorized(message: impl Into<String>, errors: Option<Vec<String>>) -> Self {
        Self::new(message, StatusCode::UNAUTHORIZED, errors)
    }

    pub fn internal_error() -> Self {
        Self {
            message: "internal server error".to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
        }
    }

    pub fn from_diesel_error(err: impl Error) -> ApiReject {
        error!("diesel error: {:?}", err);

        ApiReject::internal_error()
    }
}

impl<T> From<Jwt<T>> for ApiReject {
    fn from(value: Jwt<T>) -> Self {
        let message;
        let code;

        match value {
            Jwt::Expired => {
                message = "token expired".to_string();
                code = StatusCode::UNAUTHORIZED;
            },
            Jwt::Invalid => {
                message = "invalid token".to_string();
                code = StatusCode::UNAUTHORIZED;
            },
            Jwt::Valid(_) => {
                message = "token is valid".to_string();
                code = StatusCode::OK;
            }
        }

        ApiReject::new(message, code, None)
    }
}

impl Reject for ApiReject {}
