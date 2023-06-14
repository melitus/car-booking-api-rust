
// Register custom mods
mod loaders;
mod api;
mod config;
mod exceptions;
mod schema;
mod database;
mod utils;
mod middlewares;
mod auth;
use loaders::startup::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    run()
    .await
}
