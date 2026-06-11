use std::sync::Arc;

use super::{CacheKey, CacheStorage, CacheWriter, CachedResponse};
use http::HeaderMap;
use redb::{Database, ReadableDatabase as _, TableDefinition};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tracing")]
use tracing::instrument;

#[derive(Debug)]
pub struct ReDBCache {
    db: Arc<Database>,
}

impl ReDBCache {
    pub fn new<D: Into<Arc<Database>>>(db: D) -> Self {
        Self { db: db.into() }
    }
}

// Wrapper around `http::Uri` to implement `serde::Serialize`,
// `serde::Deserialize`, `redb::Value` and `redb::Key`.
#[derive(Debug)]
enum Uri<'a> {
    Owned(http::Uri),
    Ref(&'a http::Uri),
}

impl Serialize for Uri<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Uri::Owned(uri) => http_serde::uri::serialize(uri, serializer),
            Uri::Ref(uri) => http_serde::uri::serialize(uri, serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Uri<'_> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uri = http_serde::uri::deserialize(deserializer)?;
        Ok(Uri::Owned(uri))
    }
}

impl redb::Value for Uri<'_> {
    type SelfType<'a>
        = Uri<'a>
    where
        Self: 'a;

    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        postcard::from_bytes(data).expect("Failed to deserialize CacheKey from bytes")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        postcard::to_allocvec(value).expect("Failed to serialize CacheKey")
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("octocrab::UriWrapper")
    }
}

impl redb::Key for Uri<'_> {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}

const TABLE_CACHE_KEYS: TableDefinition<Uri, CacheKey> =
    TableDefinition::new("octocrab_cache_keys");

const TABLE_CACHE_RESPONSES: TableDefinition<Uri, CachedResponse> =
    TableDefinition::new("octocrab_cache_responses");

impl redb::Value for CacheKey {
    type SelfType<'a> = CacheKey;

    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        postcard::from_bytes(data).expect("Failed to deserialize CacheKey from bytes")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        postcard::to_allocvec(value).expect("Failed to serialize CacheKey")
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("octocrab::CacheKey")
    }
}

impl redb::Value for CachedResponse {
    type SelfType<'a> = CachedResponse;

    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        postcard::from_bytes(data).expect("Failed to deserialize CachedResponse from bytes")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        postcard::to_allocvec(value).expect("Failed to serialize CachedResponse")
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("octocrab::CachedResponse")
    }
}

#[derive(Debug)]
struct RedbWriter {
    db: Arc<Database>,
    uri: Uri<'static>,
    key: CacheKey,
    response: CachedResponse,
}

impl CacheStorage for ReDBCache {
    #[cfg_attr(feature = "tracing", instrument)]
    fn try_hit(&self, uri: &http::Uri) -> Option<CacheKey> {
        let txn = match self.db.begin_read() {
            Ok(txn) => txn,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to begin read transaction: {e}");
                return None;
            }
        };

        let table = match txn.open_table(TABLE_CACHE_KEYS) {
            Ok(table) => table,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to open cache keys table: {e}");
                return None;
            }
        };

        let cache_key = match table.get(Uri::Ref(uri)) {
            Ok(value) => value,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to read from cache keys table: {e}");
                return None;
            }
        };

        cache_key.map(|g| g.value())
    }

    #[cfg_attr(feature = "tracing", instrument)]
    fn load(&self, uri: &http::Uri) -> Option<CachedResponse> {
        let txn = match self.db.begin_read() {
            Ok(txn) => txn,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to begin read transaction: {e}");
                return None;
            }
        };

        let table = match txn.open_table(TABLE_CACHE_RESPONSES) {
            Ok(table) => table,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to open cache responses table: {e}");
                return None;
            }
        };

        let response = match table.get(Uri::Ref(uri)) {
            Ok(value) => value,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to read from cache responses table: {e}");
                return None;
            }
        };

        response.map(|g| g.value())
    }

    fn writer(&self, uri: &http::Uri, key: CacheKey, headers: HeaderMap) -> Box<dyn CacheWriter> {
        Box::new(RedbWriter {
            db: self.db.clone(),
            uri: Uri::Owned(uri.clone()),
            key,
            response: CachedResponse {
                body: Vec::new(),
                headers,
            },
        })
    }
}

impl CacheWriter for RedbWriter {
    fn write_body(&mut self, data: &[u8]) {
        self.response.body.extend_from_slice(data);
    }
}

impl Drop for RedbWriter {
    #[cfg_attr(feature = "tracing", instrument)]
    fn drop(&mut self) {
        // The whole response was received, hence the writer is dropped. We need
        // to add the response body to the cache.

        let txn = match self.db.begin_write() {
            Ok(txn) => txn,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to begin cache write transaction: {e}");
                return;
            }
        };

        {
            let mut table = match txn.open_table(TABLE_CACHE_KEYS) {
                Ok(table) => table,
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("Failed to open cache keys table: {e}");
                    return;
                }
            };

            if let Err(e) = table.insert(&self.uri, &self.key) {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to write to cache keys table: {e}");
                return;
            }

            drop(table);
        }

        {
            let mut table = match txn.open_table(TABLE_CACHE_RESPONSES) {
                Ok(table) => table,
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("Failed to open cache responses table: {e}");
                    return;
                }
            };

            if let Err(e) = table.insert(&self.uri, &self.response) {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to write to cache responses table: {e}");
                return;
            }

            drop(table);
        }

        match txn.commit() {
            Ok(_) => (),
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to commit cache write transaction: {e}");
            }
        }
    }
}
