extern crate dotenv;
extern crate env_logger;

use actix_web::{App,http,web, middleware, HttpServer};
use std::{env, io};
use env_logger::Env;
// use crate::utils::response_manager::*;
use actix_cors::Cors;
use crate::database::db::*;
use crate::api::routes::app::config_services;

// async fn health_check() -> HttpResponse {
//     // ResponseType::Ok("Welcome to car booking api").get_response_type()
//     }

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));


    let app_host: String = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port: String = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url: String = format!("{}:{}", &app_host, &app_port);
    let pool = establish_connection();
    

    let app = move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(middleware::Logger::default())
        .wrap(
            Cors::default() // allowed_origin return access-control-allow-origin: * by default
                .allowed_origin("http://127.0.0.1:3000")
                .allowed_origin("http://localhost:3000")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
        )
        .configure(config_services)
    };
    log::info!("🚀 Starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(app)
    .bind(&app_url)?
    .run()
    .await
}