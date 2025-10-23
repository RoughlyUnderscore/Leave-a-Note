use std::{fs, io};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
  fmt::{self, time::LocalTime},
  layer::SubscriberExt,
  util::SubscriberInitExt
};

use crate::config::Configuration;

pub fn init_logging(config: &Configuration) -> Result<(), io::Error> {
  fs::create_dir_all(&config.log_dir)?;

  let file_writer = RollingFileAppender::new(
    Rotation::DAILY,
    &config.log_dir,
    "leave-a-note.log");

  tracing_subscriber::registry()
    .with(fmt::layer()
      .with_ansi(false)
      .with_timer(LocalTime::rfc_3339())
      .with_writer(file_writer))
    .with(fmt::layer()
      .with_ansi(false)
      .with_timer(LocalTime::rfc_3339())
      .with_target(false))
    .init();

  Ok(())
}