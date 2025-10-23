use std::error::Error;
use axum::routing::{get};
use axum::Router;

use tracing::*;

use crate::config::Configuration;
use crate::root;

pub async fn run_app(config: Configuration) -> Result<(), Box<dyn Error>> {
  let app = Router::new()
    .route("/", get(root));

  let addr = format!("0.0.0.0:{}", config.port);
  info!("Listening on {}", addr);

  let listener = tokio::net::TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}