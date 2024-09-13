use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::{env::Env, types::ApiResponse};

pub async fn ping() -> ApiResponse {
    let Env { stage, .. } = Env::load()?;
    Ok(Json(json!({ "ok": true, "stage": stage })).into_response())
}
