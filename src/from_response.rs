use std::future::Future;

use bytes::Bytes;
use http_body::Body;
use http_body_util::BodyExt;
use snafu::ResultExt;

/// A trait for mapping from a `http::Response` to an another type.
pub trait FromResponse: Sized {
    fn from_response<B>(
        response: http::Response<B>,
    ) -> impl Future<Output = crate::Result<Self>> + Send
    where
        B: Body<Data = Bytes, Error = crate::Error> + Send;
}

impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response<B>(response: http::Response<B>) -> crate::Result<Self>
    where
        B: Body<Data = Bytes, Error = crate::Error> + Send,
    {
        let (_, body) = response.into_parts();
        let body = body.collect().await?.to_bytes();
        let de = &mut serde_json::Deserializer::from_slice(&body);
        serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu)
    }
}
