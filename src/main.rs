mod config;
mod models;
mod client;
mod handlers;

use actix_web::{web, App, HttpServer};
use crate::config::Config;
use crate::client::FireflyClient;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();
    let host = config.host.clone();
    let port = config.port;

    info!("Starting server at http://{}:{}", host, port);

    let firefly_client = web::Data::new(FireflyClient::new(config));

    HttpServer::new(move || {
        App::new()
            .app_data(firefly_client.clone())
            .service(handlers::account::get_accounts)
            .service(handlers::account::get_balance_history)
            .route("/", web::get().to(handlers::index::index))
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind((host, port))?
    .run()
    .await
}
