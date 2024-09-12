use axum::{body::Body, http::StatusCode, response::IntoResponse, Json};
use mongoose::Model;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::AppError,
    models::mock_response::{MockMethod, MockResponse},
    types::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct CreateMockPayload {
    pub name: String,
    pub method: MockMethod,
    pub response_body: Option<Value>,
    pub status_code: u16,
}

pub async fn create_mock(body: Json<CreateMockPayload>) -> ApiResponse {
    let mock = MockResponse {
        name: body.name.to_owned(),
        method: body.method.to_owned(),
        response_body: body.response_body.to_owned(),
        status_code: body.status_code,
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    Ok((StatusCode::CREATED, Json(mock)).into_response())
}
