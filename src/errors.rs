use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    EnvError(String),
    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    MethodNotAllowed(String),
}

#[allow(clippy::needless_pass_by_value)]
impl AppError {
    pub fn env_error(error: impl ToString) -> Self {
        Self::EnvError(error.to_string())
    }

    pub fn unauthorized(error: impl ToString) -> Self {
        Self::Unauthorized(error.to_string())
    }

    pub fn not_found(error: impl ToString) -> Self {
        Self::NotFound(error.to_string())
    }

    pub fn method_not_allowed(error: impl ToString) -> Self {
        Self::MethodNotAllowed(error.to_string())
    }

    pub fn internal_server_error(error: impl ToString) -> Self {
        Self::InternalServerError(error.to_string())
    }

    pub fn bad_request(error: impl ToString) -> Self {
        Self::BadRequest(error.to_string())
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        tracing::error!("[ERROR]: {self:?}");
        let error = ErrorMessage {
            error: self.to_string(),
        };
        (status, Json(error)).into_response()
    }
}
