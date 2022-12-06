use std::collections::HashMap;
use warp::{Filter, Reply};

fn collector_handler(form_id: String, body: HashMap<String, String>) -> impl Reply {
    warp::reply::with_status("collector", warp::http::StatusCode::OK)
}

pub fn get_collector() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("collector" / String)
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(collector_handler)
}
