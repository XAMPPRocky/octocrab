// Tests for caching behavior using InMemoryCache
mod cache_test_helpers;

use cache_test_helpers::etag_update_cache_impl;
use cache_test_helpers::should_cache_impl;
use octocrab::service::middleware::cache::mem::InMemoryCache;

#[tokio::test]
async fn should_cache_mem() {
    should_cache_impl(InMemoryCache::new()).await;
}

#[tokio::test]
async fn etag_update_cache_mem() {
    etag_update_cache_impl(InMemoryCache::new()).await;
}
