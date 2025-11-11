use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod config;
mod db;
mod models;
mod slug_policy;
mod routes;

use app::AppState;
use config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // logging
  tracing_subscriber::registry()
  .with(tracing_subscriber::EnvFilter::from_default_env())
  .with(tracing_subscriber::fmt::layer())
  .init();

  // load config
  let cfg = AppConfig::from_env()?;
  let addr: SocketAddr = cfg.addr.parse().expect("valid SURL_ADDR");

  // connect DB & run migrations
  let pool = db::connect_and_migrate(&cfg).await?;

  // app state
  let state = AppState::new(cfg, pool);

  // router
  let app = Router::new()
  .route("/healthz", get(|| async { "ok" }))
  .merge(routes::router(state.clone()))
  .layer(tower_http::trace::TraceLayer::new_for_http());

  tracing::info!("SURL listening on {}", addr);
  axum::Server::bind(&addr)
  .serve(app.into_make_service())
  .await?;
  Ok(())
}
