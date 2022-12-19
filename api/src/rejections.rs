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
            message: "Internal server error".to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
        }
    }

    pub fn from_diesel_error(err: impl Error) -> ApiReject {
        error!("diesel error: {:?}", err);

        ApiReject::internal_error()
    }
}

impl From<Jwt> for ApiReject {
    fn from(value: Jwt) -> Self {
        let message;
        let code;

        match value {
            Jwt::Expired => {
                message = "Token expired".to_string();
                code = StatusCode::UNAUTHORIZED;
            },
            Jwt::Invalid => {
                message = "Invalid token".to_string();
                code = StatusCode::UNAUTHORIZED;
            },
            Jwt::Valid(_) => {
                message = "Token is valid".to_string();
                code = StatusCode::OK;
            }
            _ => {
                message = "Internal server error".to_string();
                code = StatusCode::INTERNAL_SERVER_ERROR;
            }
        }

        ApiReject::new(message, code, None)
    }
}

impl Reject for ApiReject {}
