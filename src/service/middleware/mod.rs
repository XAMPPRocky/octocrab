pub mod auth_header;
pub mod base_uri;
pub mod cache;
pub mod extra_headers;
#[cfg(all(feature = "retry", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(feature = "retry")))]
pub mod retry;
