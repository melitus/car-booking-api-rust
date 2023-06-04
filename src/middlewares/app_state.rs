use crate::config::env::Config;
use crate::database::db::{PostgresPool, DBPooledConnection};
use crate::exceptions::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub env: Config,
    pub pool: PostgresPool,
}

impl AppState {
    pub fn get_conn(&self) -> Result<DBPooledConnection, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }

}

// let state = req
//         .app_data::<Data<AppState>>()
//         .expect("app_data is empty!");
//     println!("Static data: {}", state.static_data);