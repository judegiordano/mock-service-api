use service_mocker::{
    errors::AppError,
    logger,
    models::{mock_response::MockResponse, session::Session},
};
use tokio::try_join;

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    logger::init()?;
    let indexes = try_join!(MockResponse::migrate(), Session::migrate(),)
        .map_err(AppError::internal_server_error)?;
    tracing::info!("{:?}", indexes);
    Ok(())
}
