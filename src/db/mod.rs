use mongodb::{
  Collection,
  Client,
  bson::{Document}
};

use crate::config::Configuration;

pub struct DatabaseConnection {
  pub notes: Collection<Document>
}

impl DatabaseConnection {
  pub async fn load(config: &Configuration) -> mongodb::error::Result<Self> {
    let conn_string = if config.db_name.is_empty() {
      format!("mongodb://{}/?authSource={}", config.db_url, config.db_auth)
    } else {
      format!("mongodb://{}:{}@{}/?authSource={}", config.db_user, config.db_pass, config.db_url, config.db_auth)
    };
    
    let client = Client::with_uri_str(conn_string).await?;
    let db = client.database(&config.db_name);

    Ok(Self {
      notes: db.collection("notes")
    })
  }
}