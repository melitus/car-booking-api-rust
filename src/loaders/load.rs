use actix_web::{web, App,HttpResponse, middleware, HttpServer};
use std::{env, io};
use env_logger::Env;
use crate::utils::response_manager::*;

extern crate dotenv;
extern crate env_logger;

async fn health_check() -> HttpResponse {
    ResponseType::Ok("Welcome to car booking api").get_response_type()
    }

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let app = || {
        App::new()
        .wrap(middleware::Logger::default())
        .route("/health", web::get().to(health_check))
    };
    log::info!("starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(app)
    .bind(&app_url)?
    .run()
    .await
}