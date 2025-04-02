use axum::routing::{delete, get, post};

use crate::types::AppState;

mod controller;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", post(controller::create_session))
        .route(
            "/:session_id",
            get(controller::read_session), // .layer(middleware::from_fn_with_state(60, cache_response)),
        )
        .route("/:session_id", delete(controller::delete_session))
}
