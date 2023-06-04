extern crate dotenv;
extern crate env_logger;

use {
    actix_web::{App,http,web, middleware, HttpServer},
    std::{env, io},
    env_logger::Env,
    actix_cors::Cors,
    colored::Colorize,

    crate::database::db::*,
    crate::api::routes::app::config_services,
    crate::config::env::Config,
    crate::middleware::app_state::AppState,

};

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let config = Config::init();
    let pool = establish_connection(config.database_url.to_owned());

    let app_state = web::Data::new(AppState {
        pool: pool.clone(),
        env: config.clone(), 
    });

    let app_host: String = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port: String = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url: String = format!("{}:{}", &app_host, &app_port);

    log::info!("{}", "✅ Connection to the database is successful!".green());
    
    let app = move || {
        App::new()
        .app_data(app_state.clone())
        .wrap(middleware::Logger::default())
        .wrap(
            Cors::default() // allowed_origin return access-control-allow-origin: * by default
                .allowed_origin(&config.client_origin)
                // .allowed_origin("http://localhost:3000")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
        )
        .configure(config_services)
    };
    log::info!(
        "{}",
        "🚀 Starting HTTP server at http://127.0.0.1:8080"
            .bold()
            .green()
    );
    HttpServer::new(app)
    .bind(&app_url)?
    .run()
    .await
}