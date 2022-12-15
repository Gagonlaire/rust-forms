#[macro_use]
extern crate diesel;

mod routes;
mod config;
mod database;
mod models;
mod utils;
mod filters;

use config::{Config};

fn setup() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "warn,info,error");
    pretty_env_logger::init();
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    setup();

    let config = Config::default();
    let pool = database::establish_pool_connection(&config.database_url);
    let host = config.host;

    warp::serve(routes::build(config, pool)).run(host).await;
}
