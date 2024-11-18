use lambda_http::Error;
use moka::future::{Cache, CacheBuilder};
use service_mocker::{
    controllers::routes, env::Env, logger, models::session::Session, types::AppState,
};
use std::time::Duration;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    let env = Env::load()?;
    let session_cache: Cache<String, Session> = CacheBuilder::new(10_000)
        .time_to_live(Duration::from_secs(10))
        .build();
    let state = AppState { session_cache };
    let app = axum::Router::new().nest("/", routes()).with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port)).await?;
    tracing::info!("listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
