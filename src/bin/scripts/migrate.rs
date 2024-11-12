use service_mocker::{errors::AppError, logger, models::mock_response::MockResponse};

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    logger::init()?;
    let indexes = MockResponse::migrate()
        .await
        .map_err(AppError::internal_server_error)?;
    tracing::info!("{:?}", indexes);
    Ok(())
}
