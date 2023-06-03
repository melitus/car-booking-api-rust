use crate::config::env::Config;
use crate::database::db::{PostgresPool};

#[derive(Clone)]
pub struct AppState {
    pub env: Config,
    pub db: PostgresPool,
}

// let state = req
//         .app_data::<Data<AppState>>()
//         .expect("app_data is empty!");
//     println!("Static data: {}", state.static_data);