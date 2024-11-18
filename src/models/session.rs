use mongoose::{doc, types::MongooseError, DateTime, Model};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    types::{cache::SessionCache, session::Dto},
};

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
        Ok(vec![])
    }

    pub async fn get_or_cache(id: &str, cache: &SessionCache) -> Result<Self, AppError> {
        let id = id.to_string();
        if let Some(exists) = cache.get(&id).await {
            return Ok(exists);
        }
        let session = Self::read_by_id(&id)
            .await
            .map_err(|_| AppError::not_found("session not found"))?;
        cache.insert(id, session.clone()).await;
        Ok(session)
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
