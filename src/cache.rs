use moka::future::{Cache, CacheBuilder};
use std::{fmt::Debug, hash::Hash, time::Duration};

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
