use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use mongoose::Model;
use validator::Validate;

use crate::{
    errors::AppError,
    models::session::Session,
    types::{session::CreateSessionPayload, ApiResponse},
};

pub async fn create_session(body: Json<CreateSessionPayload>) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let session = Session {
        description: body.description.clone(),
        ..Default::default()
    };
    let session = session.save().await.map_err(AppError::bad_request)?;
    Ok((StatusCode::CREATED, Json(session.dto())).into_response())
}

pub async fn read_session(id: Path<String>) -> ApiResponse {
    let session = Session::read_by_id(&id.to_string())
        .await
        .map_err(AppError::not_found)?;
    Ok(Json(session.dto()).into_response())
}
