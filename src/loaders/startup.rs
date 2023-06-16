extern crate dotenv;
extern crate env_logger;

use {
    actix_web::{App,http,web, middleware, HttpServer},
    std::{env, io},
    env_logger::Env,
    colored::Colorize,

    crate::database::db::*,
    crate::api::routes::app::config_services,
    crate::config::env::Config,
    crate::middlewares::state::AppState,
    crate::middlewares::cors::cors,
};

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
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
        .wrap(cors())
        // limit the maximum amount of data that server will accept
        .app_data(web::JsonConfig::default().limit(4096))
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