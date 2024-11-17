use meme_cache::{get, set};
use mongoose::{
    doc,
    types::{ListOptions, MongooseError},
    DateTime, IndexModel, Model,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    types::{
        mock::{Dto, MockMethod, Response},
        FIVE_MINUTES_IN_MS,
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
    pub response: Response,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl MockResponse {
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = [IndexModel::builder().keys(doc! { "session": 1 }).build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub fn dto(&self) -> Dto {
        Dto {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            method: self.method.clone(),
            response: self.response.clone(),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }

    pub async fn get_or_cache(session_id: &str, id: &str) -> Result<Self, AppError> {
        let path = format!("{session_id}/{id}");
        if let Some(cached_mock) = get::<Self>(&path).await {
            return Ok(cached_mock);
        }
        let mock = Self::read_by_id(&id).await.map_err(AppError::not_found)?;
        set(&path, &mock, FIVE_MINUTES_IN_MS).await;
        Ok(mock)
    }

    pub async fn list_or_cache(session_id: &str) -> Result<Vec<Self>, AppError> {
        let path = format!("LIST/{session_id}");
        if let Some(mocks) = get::<Vec<Self>>(&path).await {
            return Ok(mocks);
        };
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
        set(&path, &mocks, FIVE_MINUTES_IN_MS).await;
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
            response: Response::default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for MockResponse {}
