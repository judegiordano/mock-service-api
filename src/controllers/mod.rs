use crate::types::AppState;

mod dev;
mod invoke;
mod mock_response;
mod session;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/dev", dev::router())
        .nest("/session", session::router())
        .nest("/mock", mock_response::router())
        .nest("/invoke", invoke::router())
}
