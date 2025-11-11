use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .with(tracing_subscriber::fmt::layer())
    .init();

  let app = Router::new().route("/healthz", get(|| async { "ok" }));

  let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
  tracing::info!("listening on {}", addr);
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
