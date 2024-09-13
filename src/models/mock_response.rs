use chrono::Utc;
use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockResponseDto {
    pub id: String,
    pub name: String,
    pub method: MockMethod,
    pub status_code: u16,
    pub response_body: Option<Value>,
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
            response_body: self.body.to_owned(),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
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
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for MockResponse {}
