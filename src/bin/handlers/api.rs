use lambda_http::Error;
use service_mocker::{controllers::routes, env::Env, logger};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    let env = Env::load()?;
    let app = axum::Router::new().nest("/", routes());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port)).await?;
    tracing::info!("listening on {:?}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}
