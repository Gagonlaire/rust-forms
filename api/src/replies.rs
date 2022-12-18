use serde_json::json;
use warp::Reply;

pub struct ApiReply {
    pub code: warp::http::StatusCode,
    pub message: String,
    pub success: bool,
}

impl ApiReply {
    pub fn new(code: Option<warp::http::StatusCode>, message: String) -> Self {
        Self {
            code: code.unwrap_or(warp::http::StatusCode::OK),
            message,
            success: true,
        }
    }

    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            code: warp::http::StatusCode::OK,
            message: message.into(),
            success: true,
        }
    }

    pub fn created(message: impl Into<String>) -> Self {
        Self {
            code: warp::http::StatusCode::CREATED,
            message: message.into(),
            success: true,
        }
    }
}

impl Reply for ApiReply {
    fn into_response(self) -> warp::reply::Response {
        let code = self.code;
        let json = json!({
            "code": self.code.as_u16(),
            "message": self.message,
            "success": self.success,
        });

        warp::reply::with_status(warp::reply::json(&json), code).into_response()
    }
}
