use axum::response::Response;

use crate::errors::AppError;

pub type ApiResponse = Result<Response, AppError>;
