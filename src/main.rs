mod loaders;
mod utils;

use loaders::load::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    run()
    .await
}
