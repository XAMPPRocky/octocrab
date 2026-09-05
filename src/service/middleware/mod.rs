pub mod auth_header;
pub mod base_uri;
pub mod cache;
pub mod extra_headers;
#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
pub mod reqwest_connector;
#[cfg(feature = "retry")]
#[cfg_attr(docsrs, doc(cfg(feature = "retry")))]
pub mod retry;
