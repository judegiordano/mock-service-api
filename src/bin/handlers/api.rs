use lambda_http::Error;
use service_mocker::{controllers::routes, logger};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    let app = axum::Router::new().nest("/api", routes());
    // bind to localhost when running cargo dev
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        tracing::info!("listening on {:?}", listener.local_addr()?);
        return Ok(axum::serve(listener, app).await?);
    }
    lambda_http::run(app).await
}
