use axum::http::Method;
use chrono::Utc;
use meme_cache::{get, set};
use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::AppError;

const MOCK_CACHE_IN_MS: i64 = 60_000;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MockMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl MockMethod {
    pub fn from_method(method: &Method) -> Result<Self, AppError> {
        let invocation_method = match method {
            &Method::OPTIONS => MockMethod::OPTIONS,
            &Method::GET => MockMethod::GET,
            &Method::POST => MockMethod::POST,
            &Method::PUT => MockMethod::PUT,
            &Method::DELETE => MockMethod::DELETE,
            &Method::HEAD => MockMethod::HEAD,
            &Method::PATCH => MockMethod::PATCH,
            _ => return Err(AppError::method_not_allowed("method not supported")),
        };
        Ok(invocation_method)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockResponseDto {
    pub id: String,
    pub name: String,
    pub method: MockMethod,
    pub status_code: u16,
    pub body: Option<Value>,
    pub headers: Option<Vec<MockHeader>>,
    pub delay_in_ms: Option<u32>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub method: MockMethod,
    pub status_code: u16,
    pub body: Option<Value>,
    pub headers: Option<Vec<MockHeader>>,
    pub delay_in_ms: Option<u32>,
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

    pub fn dto(&self) -> MockResponseDto {
        MockResponseDto {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            method: self.method.to_owned(),
            status_code: self.status_code.to_owned(),
            body: self.body.to_owned(),
            headers: self.headers.to_owned(),
            delay_in_ms: self.delay_in_ms,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }

    pub async fn get_or_cache(id: &str) -> Result<Self, AppError> {
        if let Some(cached_mock) = get::<Self>(&id).await {
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
            status_code: 200,
            method: MockMethod::GET,
            body: None,
            headers: None,
            delay_in_ms: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for MockResponse {}
