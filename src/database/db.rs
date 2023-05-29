use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use std::env;
use dotenv::dotenv;


pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Get DB Connection pool
fn init_pool(database_url: &str) -> Result<PostgresPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PostgresPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
      .expect("Error building connection pool");
    init_pool(&database_url).expect("Failed to create pool")
}
