mod dev;
mod invoke;
mod mock_response;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .nest("/dev", dev::router())
        .nest("/mock", mock_response::router())
        .nest("/invoke", invoke::router())
}
