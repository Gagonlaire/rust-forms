use std::sync::Arc;
use warp::Filter;
use crate::config::AppData;

pub fn app_data_filter(app_data: Arc<AppData>) -> impl Filter<Extract = (Arc<AppData>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || app_data.clone())
}
