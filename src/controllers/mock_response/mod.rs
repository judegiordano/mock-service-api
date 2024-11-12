use axum::routing::{delete, get, post};

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", post(controller::create_mock))
        .route("/:id", delete(controller::delete_mock))
        .route("/:id", get(controller::read_mock))
        .route("/", get(controller::list_mocks))
}
