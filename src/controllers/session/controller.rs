use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use meme_cache::{remove, set};
use mongoose::{doc, Model};
use validator::Validate;

use crate::{
    errors::AppError,
    models::session::Session,
    types::{session::CreateSessionPayload, ApiResponse, FIVE_MINUTES_IN_MS},
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

pub async fn read_session(id: Path<String>) -> ApiResponse {
    let session = Session::get_by_id(&id).await?;
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
