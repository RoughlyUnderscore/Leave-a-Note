use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
  pub port: u16,
  pub log_dir: String
}

impl Configuration {
  pub fn load() -> Self {
    Self {
      port: std::env::var("LN_PORT")
        .unwrap_or_else(|_| "45678".into())
        .parse()
        .expect("LN_PORT should be a number 1-65535"),
      
      log_dir: std::env::var("LN_LOG_DIR")
        .unwrap_or_else(|_| "logs".into())
    }
  }
}