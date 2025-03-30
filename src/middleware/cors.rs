use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};

pub async fn cors(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_static("true"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("*"),
    );
    // seconds
    headers.insert("Access-Control-Max-Age", HeaderValue::from_static("3600"));
    headers.insert(
        "Access-Control-Expose-Headers",
        HeaderValue::from_static("*"),
    );
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    response
}
