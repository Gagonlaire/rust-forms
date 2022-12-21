#[path = "auth/auth.rs"]
mod auth;
#[path = "collector/collector.rs"]
mod collector;
#[path = "forms/forms.rs"]
mod forms;

use warp::{Filter};
use crate::config::{Config};
use crate::database::{DbPool};
use crate::filters::handle_rejection;

pub fn build(config: Config, db_pool: DbPool) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let collector = collector::register();
    let auth_routes = auth::register(config.clone(), db_pool.clone());
    let forms_routes = forms::register(config, db_pool);

    warp::path!("api" / "v1" / ..)
        .and(
            collector
                .or(auth_routes)
                .or(forms_routes)
                .recover(handle_rejection)
        )
}
