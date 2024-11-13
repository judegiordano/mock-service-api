use axum::{
    extract::{Path, Request},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::time::Duration;

use crate::{
    errors::AppError,
    models::{mock_response::MockResponse, session::Session},
    types::{mock::MockMethod, ApiResponse, SessionMockParams},
};

pub async fn invoke(params: Path<SessionMockParams>, request: Request) -> ApiResponse {
    let session = Session::get_by_id(&params.session_id).await?;
    let mock_id = params.mock_id.to_string();
    let mock = MockResponse::get_or_cache(&session.id, &mock_id).await?;
    let res = mock.response;
    // sleep
    if let Some(delay) = res.delay_in_ms {
        tokio::time::sleep(Duration::from_millis(delay.into())).await;
    }
    // method
    let invocation_method = MockMethod::from_method(request.method())?;
    if invocation_method != mock.method {
        return Err(AppError::method_not_allowed(format!(
            "method should be: {:?}",
            mock.method
        )));
    };
    // status / body
    let status_code = StatusCode::from_u16(res.status_code).map_err(AppError::bad_request)?;
    let mut response = (status_code, Json(res.body)).into_response();
    // headers
    if let Some(mock_headers) = res.headers {
        let headers = response.headers_mut();
        for header in mock_headers {
            headers.append(header.parse_key()?, header.parse_value()?);
        }
    }
    Ok(response)
}
