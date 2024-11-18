use lambda_http::Error;
use service_mocker::{cache, controllers::routes, env::Env, logger, types::AppState};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    let env = Env::load()?;
    let state = AppState {
        session_cache: cache::prepare(10_000, 60_000),
        mock_cache: cache::prepare(10_000, 60_000),
    };
    let app = axum::Router::new().nest("/", routes()).with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port)).await?;
    tracing::info!("listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
