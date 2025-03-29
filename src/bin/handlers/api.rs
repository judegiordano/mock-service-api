use service_mocker::{
    cache,
    controllers::routes,
    errors::AppError,
    logger,
    middleware::{helmet::helmet, tracing::tracing_middleware},
    types::{ApiResponse, AppState, FIVE_MINUTES_IN_MS},
};

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    let state = AppState {
        session_cache: cache::prepare(10_000, FIVE_MINUTES_IN_MS),
        mock_cache: cache::prepare(10_000, FIVE_MINUTES_IN_MS),
        list_mocks_cache: cache::prepare(10_000, FIVE_MINUTES_IN_MS),
    };
    let app = axum::Router::new()
        .nest("/", routes())
        .fallback(axum::routing::any(|| async {
            ApiResponse::Err(AppError::NotFound("route not found".to_string()))
        }))
        .with_state(state)
        .layer(tracing_middleware())
        .layer(axum::middleware::from_fn(helmet));
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        tracing::info!("listening on {:?}", listener.local_addr()?);
        return Ok(axum::serve(listener, app).await?);
    }
    lambda_http::run(app).await
}
