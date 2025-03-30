use mongoose::{doc, Model};
use service_mocker::{
    errors::AppError,
    logger,
    models::{mock_response::MockResponse, session::Session},
};
use tokio::try_join;

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    logger::init()?;
    let results = try_join!(
        MockResponse::bulk_delete(doc! {}),
        Session::bulk_delete(doc! {}),
    )
    .map_err(AppError::internal_server_error)?;
    tracing::info!("{:?}", results);
    Ok(())
}
