use serde_json::json;
use warp::{Rejection, Reply};

pub fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    println!("err: {:?}", err);

    let result = json!({
        "status": 500,
        "message": "Internal Server Error".to_string(),
    });
    Ok(warp::reply::with_status(
        warp::reply::json(&result),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
