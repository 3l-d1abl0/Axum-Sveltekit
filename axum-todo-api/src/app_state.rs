use crate::config::AppConfig;
use sqlx::mysql::MySqlPool;

use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: RwLock<Option<MySqlPool>>,
}

impl AppState {
    pub async fn new() -> Arc<Self> {
        let config = crate::config::load_config();

        Arc::new(AppState {
            config,
            db_pool: RwLock::new(None),
        })
    }

    pub async fn update_db_pool(&self, pool: MySqlPool) {
        let mut db_pool = self.db_pool.write().unwrap();
        *db_pool = Some(pool);
    }
}
