use axum::{http::StatusCode, response::IntoResponse, Json};
use meme_cache::{get, set};
use mongoose::{doc, types::ListOptions, Model};

use crate::{
    env::Env,
    errors::AppError,
    models::ping::{Ping, PingDto},
    types::ApiResponse,
};

const PING_LIST_CACHE_KEY: &str = "pings";

pub async fn ping() -> ApiResponse {
    if let Some(exists) = get::<Vec<PingDto>>(PING_LIST_CACHE_KEY).await {
        return Ok(Json(exists).into_response());
    }
    let Env { stage, .. } = Env::load()?;
    let new_ping = Ping {
        stage,
        ..Default::default()
    };
    new_ping.save().await.map_err(AppError::bad_request)?;
    let pings = Ping::list(
        doc! {},
        ListOptions {
            limit: 0,
            sort: doc! { "created_at": -1 },
            ..Default::default()
        },
    )
    .await
    .map_err(AppError::bad_request)?;
    let ping_dtos = pings.iter().map(Ping::dto).collect::<Vec<_>>();
    set(PING_LIST_CACHE_KEY, &ping_dtos, 10_000).await;
    Ok((StatusCode::CREATED, Json(ping_dtos)).into_response())
}
