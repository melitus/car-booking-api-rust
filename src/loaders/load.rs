use actix_web::{web, App,HttpResponse, HttpServer};
use std::{env, io};

extern crate dotenv;
extern crate env_logger;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
    }

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    HttpServer::new(|| {
        App::new()
        .route("/health", web::get().to(health_check))
    })
    .bind(&app_url)?
    .run()
    .await
}