use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
}

pub fn load_config() -> AppConfig {
    dotenv::dotenv().ok();

    AppConfig {
        database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "N/A".to_string()),
        secret_key: env::var("SECRET_KEY").unwrap_or_else(|_| "N/A".to_string()),
    }
}
