use axum::{routing::get, Router};
use std::error::Error;

mod config;
mod app;
mod telemetry;

use config::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let config = Configuration::load();

  telemetry::init_logging(&config)?;
  app::run_app(config).await?;

  Ok(())
}

async fn root() -> &'static str {
  "hi girll"
}