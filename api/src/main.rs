#[macro_use]
extern crate diesel;

mod routes;
mod config;
mod database;
mod utils;

use config::{AppData, Config};

fn setup() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "warn,info,error");
    pretty_env_logger::init();
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    setup();

    let config = Config::default();
    let pool = database::create_connection_pool(&config.database_url);
    let app_data = AppData {
        config,
        pool
    };

    warp::serve(routes::build(&app_data)).run(app_data.config.host).await;
}
