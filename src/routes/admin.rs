use axum::Router;
use crate::app::AppState;

pub fn routes(_state: AppState) -> Router {
  // Placeholder for admin settings
  Router::new()
}

