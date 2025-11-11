use std::sync::Arc;

use crate::{config::AppConfig, db::DbPool};

#[derive(Clone)]
pub struct AppState {
  pub cfg: Arc<AppConfig>,
  pub pool: DbPool,
}

impl AppState {
  pub fn new(cfg: AppConfig, pool: DbPool) -> Self {
    Self { cfg: Arc::new(cfg), pool }
  }
}
