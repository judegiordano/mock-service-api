use axum::routing::any;

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new().route("/:session_id/:mock_id", any(controller::invoke))
}
