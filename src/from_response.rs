use snafu::ResultExt;

/// A trait for mapping from a `http::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: http::Response<hyper::Body>) -> crate::Result<Self>;
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct NoBody;

#[async_trait::async_trait]
impl FromResponse for NoBody {
    async fn from_response(response: http::Response<hyper::Body>) -> crate::Result<Self> {
        Ok(Self)
    }
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: http::Response<hyper::Body>) -> crate::Result<Self> {
        let (_, body) = response.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .context(crate::error::HyperSnafu)?;
        let de = &mut serde_json::Deserializer::from_slice(&body);
        return serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu);
    }
}
