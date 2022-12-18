use serde_json::Value;
use warp::{Filter, Reply};
use crate::filters::with_json_body;

pub fn register() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("collector" / String)
        .and(warp::post())
        .and(with_json_body(None))
        .map(collector_handler)
}

fn collector_handler(_form_id: String, _body: Value) -> impl Reply {
    warp::reply::with_status("collector", warp::http::StatusCode::OK)
}
