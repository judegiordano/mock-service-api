use axum::response::Response;

use crate::errors::AppError;

pub type ApiResponse = Result<Response, AppError>;

pub mod mock {
    use axum::http::{HeaderName, HeaderValue, Method};
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::errors::AppError;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MockHeader {
        pub key: String,
        pub value: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct Response {
        pub status_code: u16,
        pub body: Option<Value>,
        pub headers: Option<Vec<MockHeader>>,
        pub delay_in_ms: Option<u32>,
    }

    impl MockHeader {
        pub fn parse_key(&self) -> Result<HeaderName, AppError> {
            self.key.parse().map_err(AppError::bad_request)
        }

        pub fn parse_value(&self) -> Result<HeaderValue, AppError> {
            self.value.parse().map_err(AppError::bad_request)
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Dto<'a> {
        pub id: &'a str,
        pub name: &'a str,
        pub method: MockMethod,
        pub response: Response,
        pub created_at: chrono::DateTime<Utc>,
        pub updated_at: chrono::DateTime<Utc>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    pub enum MockMethod {
        GET,
        POST,
        PUT,
        DELETE,
        PATCH,
        HEAD,
        OPTIONS,
    }

    impl MockMethod {
        pub fn from_method(method: &Method) -> Result<Self, AppError> {
            let invocation_method = match *method {
                Method::OPTIONS => Self::OPTIONS,
                Method::GET => Self::GET,
                Method::POST => Self::POST,
                Method::PUT => Self::PUT,
                Method::DELETE => Self::DELETE,
                Method::HEAD => Self::HEAD,
                Method::PATCH => Self::PATCH,
                _ => return Err(AppError::method_not_allowed("method not supported")),
            };
            Ok(invocation_method)
        }
    }
}
