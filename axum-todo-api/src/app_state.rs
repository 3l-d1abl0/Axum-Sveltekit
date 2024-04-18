use crate::config::AppConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub async fn new() -> Arc<Self> {
        let config = crate::config::load_config();

        Arc::new(AppState { config })
    }
}
