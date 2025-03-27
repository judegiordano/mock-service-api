use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};

pub async fn helmet(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("Content-Security-Policy",
        HeaderValue::from_static("default-src 'self';base-uri 'self';font-src 'self' https: data:;form-action 'self';frame-ancestors 'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' https: 'unsafe-inline';upgrade-insecure-requests")
    );
    headers.insert(
        "Cross-Origin-Opener-Policy",
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        "Cross-Origin-Resource-Policy",
        HeaderValue::from_static("same-origin"),
    );
    headers.insert("Origin-Agent-Cluster", HeaderValue::from_static("?1"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("no-referrer"));
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("X-DNS-Prefetch-Control", HeaderValue::from_static("off"));
    headers.insert("X-Download-Options", HeaderValue::from_static("noopen"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("SAMEORIGIN"));
    headers.insert(
        "X-Permitted-Cross-Domain-Policies",
        HeaderValue::from_static("none"),
    );
    headers.insert("X-XSS-Protection", HeaderValue::from_static("0"));
    response
}
