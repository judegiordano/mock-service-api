use axum::routing::get;

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new().route("/ping", get(controller::ping))
}
