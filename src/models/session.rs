use chrono::Utc;
use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    errors::AppError,
    models::mock_response::MockResponse,
    types::{cache::SessionCache, mock::Dto as MockDto, session::Dto, ONE_DAY_IN_SECONDS},
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
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let exp = Duration::from_secs((ONE_DAY_IN_SECONDS * 7).into());
        let indexes = [IndexModel::builder()
            .keys(doc! { "created_at": -1 })
            .options(IndexOptions::builder().expire_after(exp).build())
            .build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
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

    pub async fn read_populated(id: &str) -> Result<PopulatedSessionDto, AppError> {
        let pipeline = vec![
            doc! {
                "$match": { "_id": id }
            },
            doc! {
                "$lookup": {
                    "from": MockResponse::name(),
                    "localField": "_id",
                    "foreignField": "session",
                    "as": "mock_responses"
                }
            },
            doc! {
                "$set":  {
                    "mock_responses": {
                        "$sortArray": {
                          "input": "$mock_responses",
                          "sortBy": { "updated_at": -1 }
                        }
                    }
                }
            },
        ];
        let response = Self::aggregate::<PopulatedSession>(pipeline, None)
            .await
            .map_err(AppError::not_found)?;
        let first = response
            .first()
            .ok_or_else(|| AppError::NotFound("no session found".to_string()))?;
        Ok(PopulatedSessionDto {
            id: first.id.to_owned(),
            description: first.description.to_owned(),
            mock_responses: first.mock_responses.iter().map(MockResponse::dto).collect(),
            created_at: first.created_at.into(),
            updated_at: first.updated_at.into(),
        })
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopulatedSession {
    #[serde(rename = "_id")]
    pub id: String,
    pub description: Option<String>,
    pub mock_responses: Vec<MockResponse>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopulatedSessionDto {
    pub id: String,
    pub description: Option<String>,
    pub mock_responses: Vec<MockDto>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
