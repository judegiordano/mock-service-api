use axum::routing::any;

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new().route("/:id", any(controller::invoke))
}
