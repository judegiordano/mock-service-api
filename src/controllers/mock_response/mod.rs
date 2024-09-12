use axum::routing::post;

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new().route("/", post(controller::create_mock))
}
