use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongoose::{doc, Model};
use validator::Validate;

use crate::{
    cache::{cache_response, FIVE_MINUTES_IN_SECONDS},
    errors::AppError,
    models::session::Session,
    types::{session::CreateSessionPayload, ApiResponse, AppState},
};

pub async fn create_session(
    State(state): State<AppState>,
    body: Json<CreateSessionPayload>,
) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let session = Session {
        description: body.description.clone(),
        ..Default::default()
    };
    let session = session.save().await.map_err(AppError::bad_request)?;
    state
        .session_cache
        .insert(session.id.to_string(), session.clone())
        .await;
    Ok((StatusCode::CREATED, Json(session.dto())).into_response())
}

pub async fn read_session(State(_): State<AppState>, session_id: Path<String>) -> ApiResponse {
    let session = Session::read_by_id(session_id.to_string())
        .await
        .map_err(|_| AppError::not_found("session not found"))?;
    let headers = cache_response(FIVE_MINUTES_IN_SECONDS)?;
    Ok((headers, Json(session.dto())).into_response())
}

pub async fn delete_session(
    State(state): State<AppState>,
    session_id: Path<String>,
) -> ApiResponse {
    let session = Session::get_or_cache(&session_id, &state.session_cache).await?;
    Session::delete(doc! { "_id": &session.id })
        .await
        .map_err(AppError::bad_request)?;
    state.session_cache.invalidate(&session.id).await;
    state.list_mocks_cache.invalidate(&session.id).await;
    Ok(Json(session.dto()).into_response())
}
