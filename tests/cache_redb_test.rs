// Tests for caching behavior using ReDBCache
mod cache_test_helpers;

use cache_test_helpers::etag_update_cache_impl;
use cache_test_helpers::should_cache_impl;
use octocrab::service::middleware::cache::redb::ReDBCache;

fn setup_cache_mem() -> ReDBCache {
    let backend = redb::backends::InMemoryBackend::new();
    let db = redb::Builder::new()
        .create_with_backend(backend)
        .expect("Failed to create ReDB database");
    ReDBCache::new(db)
}

fn setup_cache_file() -> ReDBCache {
    let file = tempfile::tempfile().expect("Failed to create temporary file");
    let backend =
        redb::backends::FileBackend::new(file).expect("Failed to create ReDB file backend");
    let db = redb::Builder::new()
        .create_with_backend(backend)
        .expect("Failed to create ReDB database");
    ReDBCache::new(db)
}

#[tokio::test]
async fn should_cache_redb_mem_backend() {
    should_cache_impl(setup_cache_mem()).await;
}

#[tokio::test]
async fn etag_update_cache_redb_mem_backend() {
    etag_update_cache_impl(setup_cache_mem()).await;
}

#[tokio::test]
async fn should_cache_redb_file_backend() {
    should_cache_impl(setup_cache_file()).await;
}

#[tokio::test]
async fn etag_update_cache_redb_file_backend() {
    etag_update_cache_impl(setup_cache_file()).await;
}
