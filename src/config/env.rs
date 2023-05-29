use dotenv::dotenv;
use std::env;

/// Init env variables
pub fn init_env() {
    dotenv().ok().expect("Env init error");
}

/// Determine if runtime environment is dev|prod|etc.
pub fn is_dev() -> bool {
    let env = env::var("RUN_ENV").expect("Runtime env not set");
    env == "dev"
}

/// Get host port tuple for starting Actix server
pub fn get_host_port() -> (String, u16) {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT")
        .expect("Port not set")
        .parse::<u16>()
        .unwrap();

    (host, port)
}

/// Get app name configured
pub fn get_app_name() -> String {
    env::var("APP_NAME").unwrap_or("codefee-works-api".into())
}

pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub url: String,
    pub secret_key: String,
    pub database_url: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
        let secret_key = std::env::var("SECRET_KEY").expect("set SECRET_KEY");
        let url = std::env::var("URL").expect("set URL");
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<i32>()
            .unwrap();

        Self {
            url,
            database_url,
            secret_key,
            host,
            port,
        }
    }
}
    // to use 
    // let config = config::ServerConfig::from_env();
