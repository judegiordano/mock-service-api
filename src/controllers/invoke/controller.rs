use axum::{
    extract::{Path, Request},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::time::Duration;

use crate::{
    errors::AppError,
    models::mock_response::{MockMethod, MockResponse},
    types::ApiResponse,
};

pub async fn invoke(mock_id: Path<String>, req: Request) -> ApiResponse {
    let mock_id = mock_id.to_string();
    let mock = MockResponse::get_or_cache(&mock_id).await?;
    // sleep
    if let Some(delay) = mock.delay_in_ms {
        tokio::time::sleep(Duration::from_millis(delay.into())).await;
    }
    // method
    let invocation_method = MockMethod::from_method(req.method())?;
    if invocation_method != mock.method {
        return Err(AppError::method_not_allowed(format!(
            "method should be: {:?}",
            mock.method
        )));
    };
    // status / body
    let status_code = StatusCode::from_u16(mock.status_code).map_err(AppError::bad_request)?;
    let mut response = (status_code, Json(mock.body)).into_response();
    // headers
    if let Some(mock_headers) = mock.headers {
        let headers = response.headers_mut();
        for header in mock_headers {
            headers.append(header.parse_key()?, header.parse_value()?);
        }
    }
    Ok(response)
}
