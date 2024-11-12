use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};

use crate::types::session::Dto;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    #[serde(rename = "_id")]
    pub id: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Session {
    #[allow(dead_code)]
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = vec![IndexModel::builder()
            .keys(doc! {})
            .options(IndexOptions::builder().build())
            .build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub fn dto(&self) -> Dto {
        Dto {
            id: self.id.clone(),
            description: self.description.clone(),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            description: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Session {}
