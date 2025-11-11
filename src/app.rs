use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::routes;
use crate::{config::AppConfig, db::DbPool};

#[derive(Clone)]
pub struct AppState {
  #[allow(dead_code)]
  pub cfg: Arc<AppConfig>,
  #[allow(dead_code)]
  pub pool: DbPool,
}

impl AppState {
  pub fn new(cfg: AppConfig, pool: DbPool) -> Self {
    Self {
      cfg: Arc::new(cfg),
      pool,
    }
  }
}

pub fn router(state: AppState) -> Router {
  Router::new()
    .route("/healthz", get(|| async { "ok" }))
    .merge(routes::router(state.clone()))
    .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::{
    body::Body,
    http::{Request, StatusCode},
  };
  use http_body_util::BodyExt;
  use tower::ServiceExt;

  use crate::config::AppConfig;
  use crate::db::DbPool;

  #[cfg(feature = "postgres")]
  use sqlx::postgres::PgPoolOptions;
  #[cfg(feature = "sqlite")]
  use sqlx::sqlite::SqlitePoolOptions;

  #[tokio::test]
  async fn healthz_returns_ok() {
    let cfg = test_config();
    let pool = test_pool(&cfg).await;
    let state = AppState::new(cfg, pool);
    let app = router(state);

    let response = app
      .oneshot(
        Request::builder()
          .uri("/healthz")
          .body(Body::empty())
          .unwrap(),
      )
      .await
      .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body.as_ref(), b"ok");
  }

  fn test_config() -> AppConfig {
    AppConfig {
      addr: "127.0.0.1:0".into(),
      database_url: test_database_url(),
      pool_max: 1,
      force_status_301: true,
      reserved_slugs: Vec::new(),
      analytics_mode: "count_only".into(),
      slug_regex: "^[A-Za-z0-9]{5,10}$".into(),
      ip_anonymize: true,
      proxy_trust_cidrs: vec!["127.0.0.1/32".into()],
      admin_token: "test-token".into(),
    }
  }

  #[cfg(feature = "postgres")]
  fn test_database_url() -> String {
    "postgres://postgres:postgres@localhost:5432/surl".into()
  }

  #[cfg(feature = "sqlite")]
  fn test_database_url() -> String {
    "sqlite::memory:".into()
  }

  #[cfg(feature = "postgres")]
  async fn test_pool(cfg: &AppConfig) -> DbPool {
    PgPoolOptions::new()
      .max_connections(1)
      .connect_lazy(&cfg.database_url)
      .expect("lazy pg pool")
  }

  #[cfg(feature = "sqlite")]
  async fn test_pool(cfg: &AppConfig) -> DbPool {
    SqlitePoolOptions::new()
      .max_connections(1)
      .connect(&cfg.database_url)
      .await
      .expect("sqlite pool")
  }
}
