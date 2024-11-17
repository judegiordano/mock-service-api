use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use meme_cache::{remove, set};
use mongoose::{doc, Model};
use validator::Validate;

use crate::{
    errors::AppError,
    models::{mock_response::MockResponse, session::Session},
    types::{
        mock::{CreateMockPayload, ParseMethod},
        ApiResponse, SessionMockParams, FIVE_MINUTES_IN_MS,
    },
};

pub async fn create_mock(session_id: Path<String>, body: Json<CreateMockPayload>) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let session = Session::get_or_cache(&session_id).await?;
    let mock = MockResponse {
        session: session.id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        method: body.method.try_from_string()?,
        response: body.response.clone(),
        ..Default::default()
    };
    let mock = mock.save().await.map_err(AppError::bad_request)?;
    let path = format!("{}/{}", session.id, mock.id);
    set(&path, &mock, FIVE_MINUTES_IN_MS).await;
    let list_all_path = format!("LIST/{}", session.id);
    remove(&list_all_path).await;
    Ok((StatusCode::CREATED, Json(mock.dto())).into_response())
}

pub async fn delete_mock(params: Path<SessionMockParams>) -> ApiResponse {
    let session = Session::get_or_cache(&params.session_id).await?;
    let removed = MockResponse::delete(doc! {
        "_id": &params.mock_id,
        "session": &session.id
    })
    .await
    .map_err(AppError::bad_request)?;
    let path = format!("{}/{}", session.id, params.mock_id);
    remove(&path).await;
    let list_all_path = format!("LIST/{}", session.id);
    remove(&list_all_path).await;
    Ok(Json(removed).into_response())
}

pub async fn list_mocks(session_id: Path<String>) -> ApiResponse {
    let session = Session::get_or_cache(&session_id).await?;
    let mocks = MockResponse::list_or_cache(&session.id).await?;
    let mocks = mocks.iter().map(MockResponse::dto).collect::<Vec<_>>();
    Ok((Json(mocks)).into_response())
}
