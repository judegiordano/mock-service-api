use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use meme_cache::{clear, get, set};
use mongoose::{doc, types::ListOptions, Model};
use validator::Validate;

use crate::{
    errors::AppError,
    models::{mock_response::MockResponse, session::Session},
    types::{
        mock::{CreateMockPayload, Dto, ParseMethod},
        ApiResponse, SessionMockParams,
    },
};

const ALL_MOCKS_CACHE_TTL: i64 = 60_000;

pub async fn create_mock(session_id: Path<String>, body: Json<CreateMockPayload>) -> ApiResponse {
    body.validate().map_err(AppError::bad_request)?;
    let session = Session::get_by_id(&session_id).await?;
    let mock = MockResponse {
        session: session.id,
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

pub async fn delete_mock(params: Path<SessionMockParams>) -> ApiResponse {
    let session = Session::get_by_id(&params.session_id).await?;
    let removed = MockResponse::delete(doc! {
        "_id": params.mock_id.to_string(),
        "session": &session.id
    })
    .await
    .map_err(AppError::bad_request)?;
    clear().await;
    Ok(Json(removed).into_response())
}

pub async fn list_mocks(session_id: Path<String>) -> ApiResponse {
    let session = Session::get_by_id(&session_id).await?;
    if let Some(mocks) = get::<Vec<Dto>>(&session.id).await {
        return Ok((Json(mocks)).into_response());
    };
    let mocks = MockResponse::list(
        doc! {
            "session": &session.id
        },
        ListOptions {
            limit: 0,
            sort: doc! { "created_at": -1 },
            ..ListOptions::default()
        },
    )
    .await
    .map_err(AppError::bad_request)?;
    let mocks = mocks.iter().map(MockResponse::dto).collect::<Vec<_>>();
    set(&session.id, &mocks, ALL_MOCKS_CACHE_TTL).await;
    Ok((Json(mocks)).into_response())
}
