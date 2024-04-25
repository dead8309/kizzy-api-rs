use std::env;

use actix_web::{get, middleware::Logger, App, HttpServer, Responder};
use dotenv::dotenv;

mod routes;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info,api=info");
    }
    dotenv().ok();
    env_logger::init();
    let port: u16 = env::var("PORT")
        .unwrap()
        .parse::<u16>()
        .expect("PORT not provided");
    let address = env::var("HOST").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(home)
            .configure(routes::config::config)
            .wrap(Logger::default())
    })
    .bind((address, port))?
    .run()
    .await
}

#[get("/")]
async fn home() -> impl Responder {
    format!("Rest Apis for Kizzy App")
}
