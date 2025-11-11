use axum::Router;
use crate::app::AppState;

pub fn routes(_state: AppState) -> Router {
  // Placeholder: filled in by later tasks (POST /shorten, /me/links, etc.)
  Router::new()
}
