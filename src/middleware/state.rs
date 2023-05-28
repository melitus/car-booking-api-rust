use crate::error::AppError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type AppConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

impl AppState {
    pub fn get_conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}
