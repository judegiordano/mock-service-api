use axum::{
    extract::{Path, Request},
    http::{Method, StatusCode},
    response::IntoResponse,
    Json,
};
use meme_cache::{get, set};
use mongoose::Model;

use crate::{
    errors::AppError,
    models::mock_response::{MockMethod, MockResponse},
    types::ApiResponse,
};

const MOCK_CACHE_IN_MS: i64 = 60_000;

async fn get_or_cache(mock_id: &str) -> Result<MockResponse, AppError> {
    if let Some(cached_mock) = get::<MockResponse>(&mock_id).await {
        return Ok(cached_mock);
    }
    let mock = MockResponse::read_by_id(&mock_id)
        .await
        .map_err(AppError::not_found)?;
    set(&mock_id, &mock, MOCK_CACHE_IN_MS).await;
    Ok(mock)
}

pub async fn invoke(mock_id: Path<String>, req: Request) -> ApiResponse {
    let mock_id = mock_id.to_string();
    let mock = get_or_cache(&mock_id).await?;
    let invocation_method = match req.method() {
        &Method::OPTIONS => MockMethod::OPTIONS,
        &Method::GET => MockMethod::GET,
        &Method::POST => MockMethod::POST,
        &Method::PUT => MockMethod::PUT,
        &Method::DELETE => MockMethod::DELETE,
        &Method::HEAD => MockMethod::HEAD,
        &Method::PATCH => MockMethod::PATCH,
        _ => return Err(AppError::method_not_allowed("method not supported")),
    };
    if invocation_method != mock.method {
        return Err(AppError::method_not_allowed(format!(
            "method should be: {:?}",
            mock.method
        )));
    };
    let status_code = StatusCode::from_u16(mock.status_code).map_err(AppError::bad_request)?;
    let body = mock.response_body;
    Ok((status_code, Json(body)).into_response())
}
