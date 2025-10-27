use std::error::Error;

mod config;
mod app;
mod telemetry;
mod db;
mod routes;

use crate::{config::Configuration, db::DatabaseConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let config = Configuration::load();
  let db = DatabaseConnection::load(&config).await?;

  telemetry::init_logging(&config)?;
  app::run_app(config, db).await?;

  Ok(())
}