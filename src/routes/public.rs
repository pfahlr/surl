use axum::{routing::get, Router};
use crate::app::AppState;

pub fn routes(_state: AppState) -> Router {
  // Root landing page placeholder for now
  Router::new().route("/", get(landing))
}

async fn landing() -> &'static str {
  "SURL — ultra-fast short links (bootstrap). Try GET /healthz"
}
