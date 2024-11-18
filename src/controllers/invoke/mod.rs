use axum::routing::any;

use crate::types::AppState;

mod controller;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/:session_id/:mock_id", any(controller::invoke))
}
