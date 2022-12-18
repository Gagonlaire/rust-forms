use std::fmt::{Debug};
use diesel::r2d2::PoolError;
use log::error;
use warp::http::StatusCode;
use warp::reject::Reject;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub code: StatusCode,
    pub errors: Option<Vec<String>>,
}

impl AppError {
    pub fn new(message: String, code: StatusCode, errors: Option<Vec<String>>) -> Self {
        Self {
            message,
            code,
            errors,
        }
    }

    pub fn from_diesel_error(err: impl std::error::Error) -> AppError {
        error!("diesel error: {:?}", err);

        AppError::new(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        )
    }
}

impl Reject for AppError {}
