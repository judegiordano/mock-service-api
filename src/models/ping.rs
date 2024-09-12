use chrono::Utc;
use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};

use crate::env::Stage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ping {
    #[serde(rename = "_id")]
    pub id: String,
    pub stage: Stage,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PingDto {
    pub id: String,
    pub stage: Stage,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Ping {
    #[allow(dead_code)]
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = vec![IndexModel::builder()
            .keys(doc! {})
            .options(IndexOptions::builder().build())
            .build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub fn dto(&self) -> PingDto {
        PingDto {
            id: self.id.to_string(),
            stage: self.stage.clone(),
            created_at: self.created_at.to_chrono(),
            updated_at: self.updated_at.to_chrono(),
        }
    }
}

impl Default for Ping {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            stage: Stage::Local,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Ping {}
