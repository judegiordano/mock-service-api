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
    pub status_code: u16,
}

pub async fn create_mock(body: Json<CreateMockPayload>) -> ApiResponse {
    let mock = MockResponse {
        name: body.name.to_owned(),
        method: body.method.to_owned(),
        body: body.body.to_owned(),
        headers: body.headers.to_owned(),
        status_code: body.status_code,
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    Ok((StatusCode::CREATED, Json(mock.dto())).into_response())
}
