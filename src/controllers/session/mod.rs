use axum::routing::{delete, get, post};

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", post(controller::create_session))
        .route("/:id", get(controller::read_session))
        .route("/:id", delete(controller::delete_session))
}
