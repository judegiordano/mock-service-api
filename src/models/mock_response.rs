use meme_cache::{get, set};
use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    types::mock::{Dto, MockMethod, Response},
};

const MOCK_CACHE_IN_MS: i64 = 60_000;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub method: MockMethod,
    pub response: Response,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl MockResponse {
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
            id: &self.id,
            name: &self.name,
            method: self.method.clone(),
            response: self.response.clone(),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }

    pub async fn get_or_cache(id: &str) -> Result<Self, AppError> {
        if let Some(cached_mock) = get::<Self>(id).await {
            return Ok(cached_mock);
        }
        let mock = Self::read_by_id(&id).await.map_err(AppError::not_found)?;
        set(&id, &mock, MOCK_CACHE_IN_MS).await;
        Ok(mock)
    }
}

impl Default for MockResponse {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            name: String::default(),
            method: MockMethod::GET,
            response: Default::default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for MockResponse {}
