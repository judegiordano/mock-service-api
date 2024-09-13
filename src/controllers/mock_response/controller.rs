use axum::{http::StatusCode, response::IntoResponse, Json};
use mongoose::Model;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    errors::AppError,
    models::mock_response::{MockHeader, MockMethod, MockResponse},
    types::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct CreateMockPayload {
    pub name: String,
    pub method: MockMethod,
    pub body: Option<Value>,
    pub headers: Option<Vec<MockHeader>>,
    pub delay_in_ms: Option<u32>,
    pub status_code: u16,
}

pub async fn create_mock(body: Json<CreateMockPayload>) -> ApiResponse {
    let mock = MockResponse {
        name: body.name.clone(),
        method: body.method.clone(),
        body: body.body.clone(),
        headers: body.headers.clone(),
        status_code: body.status_code,
        delay_in_ms: body.delay_in_ms,
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    Ok((StatusCode::CREATED, Json(mock.dto())).into_response())
}
