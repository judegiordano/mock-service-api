use axum::routing::{delete, get, post};

use crate::types::AppState;

mod controller;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/:session_id", post(controller::create_mock))
        .route("/:session_id/:mock_id", delete(controller::delete_mock))
        .route("/:session_id", get(controller::list_mocks))
        .route("/:session_id/:mock_id", get(controller::read_mock))
}
