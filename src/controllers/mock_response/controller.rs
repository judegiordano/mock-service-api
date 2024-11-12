use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use meme_cache::{clear, get, set};
use mongoose::{doc, types::ListOptions, Model};
use validator::Validate;

use crate::{
    errors::AppError,
    models::mock_response::MockResponse,
    types::{
        mock::{CreateMockPayload, Dto, ParseMethod},
        ApiResponse,
    },
};

const ALL_MOCKS_CACHE_KEY: &str = "all-mocks";
const ALL_MOCKS_CACHE_TTL: i64 = 60_000;

pub async fn create_mock(body: Json<CreateMockPayload>) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let mock = MockResponse {
        name: body.name.clone(),
        description: body.description.clone(),
        method: body.method.try_from_string()?,
        response: body.response.clone(),
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    clear().await;
    Ok((StatusCode::CREATED, Json(mock.dto())).into_response())
}

pub async fn delete_mock(id: Path<String>) -> ApiResponse {
    let removed = MockResponse::delete(doc! { "_id": id.to_string() })
        .await
        .map_err(AppError::bad_request)?;
    clear().await;
    Ok(Json(removed).into_response())
}

pub async fn read_mock(id: Path<String>) -> ApiResponse {
    let mock = MockResponse::get_or_cache(&id.to_string()).await?;
    Ok(Json(mock).into_response())
}

pub async fn list_mocks() -> ApiResponse {
    if let Some(mocks) = get::<Vec<Dto>>(ALL_MOCKS_CACHE_KEY).await {
        return Ok((Json(mocks)).into_response());
    };
    let mocks = MockResponse::list(
        Default::default(),
        ListOptions {
            limit: 0,
            sort: doc! { "created_at": -1 },
            ..ListOptions::default()
        },
    )
    .await
    .map_err(AppError::bad_request)?;
    let mocks = mocks.iter().map(MockResponse::dto).collect::<Vec<_>>();
    set(ALL_MOCKS_CACHE_KEY, &mocks, ALL_MOCKS_CACHE_TTL).await;
    Ok((Json(mocks)).into_response())
}
