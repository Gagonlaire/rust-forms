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
    let pool = database::establish_connection(&config.database_url);
    let test = config.host;

    warp::serve(routes::build(config, pool)).run(test).await;
}
