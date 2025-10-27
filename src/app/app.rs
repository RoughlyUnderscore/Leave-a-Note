use std::error::Error;
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::{get};
use axum::{BoxError, Router};

use tower::ServiceBuilder;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tracing::*;

use crate::config::Configuration;
use crate::db::DatabaseConnection;

use crate::routes::ping;
use crate::routes::auth;

pub async fn run_app(config: Configuration, db: DatabaseConnection) -> Result<(), Box<dyn Error>> {
  let app = Router::new()
    .route("/ping", get(ping))
    .route("/auth", auth::create_auth_router())

    .layer(ServiceBuilder::new()
      .layer(HandleErrorLayer::new(|err: BoxError| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled error {}", err))
      }))
      .layer(BufferLayer::new(1024))
      .layer(RateLimitLayer::new(3, Duration::from_secs(60)))
    );

  let addr = format!("0.0.0.0:{}", config.port);
  info!("Listening on {}", addr);

  let listener = tokio::net::TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}