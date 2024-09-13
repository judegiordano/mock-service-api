use axum::{http::StatusCode, response::IntoResponse, Json};
use mongoose::Model;
use serde::Deserialize;

use crate::{
    errors::AppError,
    models::mock_response::MockResponse,
    types::{
        mock::{MockMethod, Response},
        ApiResponse,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateMockPayload {
    pub name: String,
    pub method: MockMethod,
    pub response: Response,
}

pub async fn create_mock(body: Json<CreateMockPayload>) -> ApiResponse {
    let mock = MockResponse {
        name: body.name.clone(),
        method: body.method.clone(),
        response: body.response.clone(),
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    Ok((StatusCode::CREATED, Json(mock.dto())).into_response())
}
