use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use meme_cache::{remove, set};
use mongoose::{doc, Model};
use validator::Validate;

use crate::{
    errors::AppError,
    models::session::Session,
    types::{session::CreateSessionPayload, ApiResponse, AppState, FIVE_MINUTES_IN_MS},
};

pub async fn create_session(body: Json<CreateSessionPayload>) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let session = Session {
        description: body.description.clone(),
        ..Default::default()
    };
    let session = session.save().await.map_err(AppError::bad_request)?;
    set(&session.id, &session, FIVE_MINUTES_IN_MS).await;
    Ok((StatusCode::CREATED, Json(session.dto())).into_response())
}

pub async fn read_session(id: Path<String>, State(state): State<AppState>) -> ApiResponse {
    let cache = state.session_cache;
    if let Some(exists) = cache.get(&id.to_string()).await {
        return Ok(Json(exists.dto()).into_response());
    }
    let session = Session::get_by_id(&id).await?;
    cache.insert(id.to_string(), session.clone()).await;
    Ok(Json(session.dto()).into_response())
}

pub async fn delete_session(id: Path<String>) -> ApiResponse {
    let session = Session::get_or_cache(&id).await?;
    Session::delete(doc! { "_id": &session.id })
        .await
        .map_err(AppError::bad_request)?;
    remove(&session.id).await;
    Ok(Json(session.dto()).into_response())
}
