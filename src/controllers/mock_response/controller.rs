use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongoose::{doc, Model};
use validator::Validate;

use crate::{
    errors::AppError,
    models::{mock_response::MockResponse, session::Session},
    types::{
        mock::{CreateMockPayload, ParseMethod},
        session::SessionMockParams,
        ApiResponse, AppState,
    },
};

pub async fn create_mock(
    State(state): State<AppState>,
    session_id: Path<String>,
    body: Json<CreateMockPayload>,
) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let session = Session::get_or_cache(&session_id, &state.session_cache).await?;
    let mock = MockResponse {
        session: session.id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        method: body.method.try_from_string()?,
        response: body.response.clone(),
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    state
        .mock_cache
        .insert(mock.id.to_string(), mock.clone())
        .await;
    // TODO: update list
    Ok((StatusCode::CREATED, Json(mock.dto())).into_response())
}

pub async fn delete_mock(
    State(state): State<AppState>,
    params: Path<SessionMockParams>,
) -> ApiResponse {
    let session = Session::get_or_cache(&params.session_id, &state.session_cache).await?;
    let mock = MockResponse::get_or_cache(&params.mock_id, &state.mock_cache).await?;
    MockResponse::delete(doc! {
        "_id": &mock.id,
        "session": &session.id
    })
    .await
    .map_err(AppError::bad_request)?;
    state.mock_cache.remove(&params.mock_id.to_string()).await;
    state
        .list_mocks_cache
        .invalidate(&session.id.to_string())
        .await;
    Ok(Json(mock).into_response())
}

pub async fn list_mocks(State(state): State<AppState>, session_id: Path<String>) -> ApiResponse {
    let session = Session::get_or_cache(&session_id, &state.session_cache).await?;
    let mocks = MockResponse::list_or_cache(&session.id, &state.list_mocks_cache).await?;
    let mocks = mocks.iter().map(MockResponse::dto).collect::<Vec<_>>();
    Ok((Json(mocks)).into_response())
}

pub async fn read_mock(
    State(state): State<AppState>,
    params: Path<SessionMockParams>,
) -> ApiResponse {
    Session::get_or_cache(&params.session_id, &state.session_cache).await?;
    let mock = MockResponse::get_or_cache(&params.mock_id, &state.mock_cache).await?;
    Ok((Json(mock.dto())).into_response())
}
