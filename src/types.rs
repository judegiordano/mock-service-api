use axum::response::Response;
use cache::{MockCache, SessionCache};

use crate::errors::AppError;

pub type ApiResponse = Result<Response, AppError>;

pub const FIVE_MINUTES_IN_MS: i64 = (1_000 * 60) * 5;

#[derive(Clone)]
pub struct AppState {
    pub session_cache: SessionCache,
    pub mock_cache: MockCache,
}

pub mod cache {
    use moka::future::Cache;

    use crate::models::{mock_response::MockResponse, session::Session};

    pub type SessionCache = Cache<String, Session>;
    pub type MockCache = Cache<String, MockResponse>;
    pub type ListMockCache = Cache<String, Vec<MockResponse>>;
}

pub mod mock {
    use axum::http::{HeaderName, HeaderValue, Method};
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use validator::Validate;

    use crate::errors::AppError;

    #[derive(Debug, Serialize, Deserialize, Clone, Validate)]
    pub struct MockHeader {
        #[validate(length(min = 1, max = 50, message = "length should be between 1 and 50"))]
        pub key: String,
        #[validate(length(min = 1, max = 500, message = "length should be between 1 and 50"))]
        pub value: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
    pub struct Response {
        #[validate(range(min = 100, max = 599, message = "range should be between 100 and 599"))]
        pub status_code: u16,
        // TODO: this should probably be custom deeply validated, for max array length and such
        pub body: Option<Value>,
        #[validate(nested)]
        pub headers: Option<Vec<MockHeader>>,
        #[validate(range(
            min = 1,
            max = 10_000,
            message = "range should be between 1 and 10,000"
        ))]
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
    pub struct Dto {
        pub id: String,
        pub name: String,
        pub description: Option<String>,
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

    pub trait ParseMethod {
        fn try_from_string(&self) -> Result<MockMethod, AppError>;
    }

    impl ParseMethod for String {
        fn try_from_string(&self) -> Result<MockMethod, AppError> {
            match self.to_uppercase().trim() {
                "GET" => Ok(MockMethod::GET),
                "POST" => Ok(MockMethod::POST),
                "PUT" => Ok(MockMethod::PUT),
                "DELETE" => Ok(MockMethod::DELETE),
                "PATCH" => Ok(MockMethod::PATCH),
                "HEAD" => Ok(MockMethod::HEAD),
                "OPTIONS" => Ok(MockMethod::OPTIONS),
                _ => Err(AppError::BadRequest("method not supported".to_string())),
            }
        }
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct CreateMockPayload {
        #[validate(length(
            min = 1,
            max = 100,
            message = "length should be between 1 and 100 characters"
        ))]
        pub name: String,
        #[validate(length(
            min = 0,
            max = 1000,
            message = "length should be between 0 and 1000 characters"
        ))]
        pub description: Option<String>,
        #[validate(length(
            min = 1,
            max = 10,
            message = "length should be between 1 and 10 characters"
        ))]
        pub method: String,
        #[validate(nested)]
        pub response: Response,
    }
}

pub mod session {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SessionMockParams {
        pub session_id: String,
        pub mock_id: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Dto {
        pub id: String,
        pub description: Option<String>,
        pub created_at: chrono::DateTime<Utc>,
        pub updated_at: chrono::DateTime<Utc>,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct CreateSessionPayload {
        #[validate(length(
            min = 0,
            max = 1000,
            message = "length should be between 0 and 1000 characters"
        ))]
        pub description: Option<String>,
    }
}
