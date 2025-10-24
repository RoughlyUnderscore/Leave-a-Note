use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
  pub port: u16,
  pub log_dir: String,

  pub db_url: String,
  pub db_user: String,
  pub db_pass: String,
  pub db_name: String,
  pub db_auth: String,
}

impl Configuration {
  pub fn load() -> Self {
    Self {
      port: std::env::var("LN_PORT")
        .unwrap_or_else(|_| "45678".into())
        .parse()
        .expect("LN_PORT should be a number 1-65535"),
      
      log_dir: std::env::var("LN_LOG_DIR")
        .unwrap_or_else(|_| "logs".into()),
      
      db_url: std::env::var("LN_DB_URL")
        .unwrap_or_else(|_| "localhost:27017".into()),
      
      db_user: std::env::var("LN_DB_USER")
        .unwrap_or_else(|_| "root".into()),
      
      db_pass: std::env::var("LN_DB_PASS")
        .unwrap_or_else(|_| "".into()),
      
      db_name: std::env::var("LN_DB_NAME")
        .unwrap_or_else(|_| "leave_a_note".into()),
      
      db_auth: std::env::var("LN_DB_AUTH")
        .unwrap_or_else(|_| std::env::var("LN_DB_AUTH_SOURCE")
          .unwrap_or_else(|_| "admin".into())),
    }
  }
}