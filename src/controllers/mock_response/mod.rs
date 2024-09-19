use axum::routing::{get, post};

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", post(controller::create_mock))
        .route("/", get(controller::list_mocks))
}
