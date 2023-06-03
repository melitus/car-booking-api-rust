
// Register custom mods
mod loaders;
mod api;
mod config;
mod exceptions;
mod schema;
mod database;
mod utils;
mod middleware;

use loaders::load::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    run()
    .await
}
