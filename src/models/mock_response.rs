use std::time::Duration;

use mongoose::{
    doc,
    types::{ListOptions, MongooseError},
    DateTime, IndexModel, IndexOptions, Model,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    types::{
        cache::{ListMockCache, MockCache},
        mock::{Dto, MockMethod, Query, Response},
        ONE_DAY_IN_SECONDS,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub session: String,
    pub name: String,
    pub description: Option<String>,
    pub method: MockMethod,
    pub params: Vec<Query>,
    pub response: Response,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl MockResponse {
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let exp = Duration::from_secs((ONE_DAY_IN_SECONDS * 7).into());
        let indexes = [
            IndexModel::builder().keys(doc! { "session": 1 }).build(),
            IndexModel::builder()
                .keys(doc! { "created_at": -1 })
                .options(IndexOptions::builder().expire_after(exp).build())
                .build(),
        ];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub fn dto(&self) -> Dto {
        Dto {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            method: self.method.clone(),
            params: self.params.clone(),
            response: self.response.clone(),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }

    pub async fn get_or_cache(id: &str, cache: &MockCache) -> Result<Self, AppError> {
        let id = id.to_string();
        if let Some(exists) = cache.get(&id).await {
            return Ok(exists);
        }
        let mock = Self::read_by_id(&id)
            .await
            .map_err(|_| AppError::not_found("mock not found"))?;
        cache.insert(id, mock.clone()).await;
        Ok(mock)
    }

    pub async fn list_or_cache(
        session_id: &str,
        cache: &ListMockCache,
    ) -> Result<Vec<Self>, AppError> {
        let id = session_id.to_string();
        if let Some(exists) = cache.get(&id).await {
            return Ok(exists);
        }
        let mocks = Self::list(
            doc! { "session": &session_id },
            ListOptions {
                limit: 0,
                sort: doc! { "created_at": -1 },
                ..ListOptions::default()
            },
        )
        .await
        .map_err(AppError::not_found)?;
        cache.insert(id, mocks.clone()).await;
        Ok(mocks)
    }
}

impl Default for MockResponse {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            session: String::default(),
            name: String::default(),
            description: None,
            method: MockMethod::GET,
            params: Vec::default(),
            response: Response::default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for MockResponse {}
