use axum::http::HeaderMap;
use moka::future::{Cache, CacheBuilder};
use std::{fmt::Debug, hash::Hash, time::Duration};

use crate::errors::AppError;

pub fn prepare<
    T: std::cmp::Eq + Hash + Send + Debug + Sync + 'static,
    V: Clone + Send + Sync + 'static,
>(
    capacity: u64,
    ttl_ms: u64,
) -> Cache<T, V> {
    CacheBuilder::new(capacity)
        .time_to_live(Duration::from_millis(ttl_ms))
        .eviction_listener(|key, _, cause| {
            tracing::debug!("[EVICTING {key:?}]: [CAUSE]: {cause:?}");
        })
        .build()
}

pub fn cache_response(seconds: u16) -> Result<HeaderMap, AppError> {
    let age = format!("max-age={seconds}, public");
    let mut headers = HeaderMap::new();
    headers.insert("Cache-Control", age.parse().map_err(AppError::bad_request)?);
    Ok(headers)
}
