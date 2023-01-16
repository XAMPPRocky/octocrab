use snafu::ResultExt;

/// A trait for mapping from a `hyper::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: hyper::Response<hyper::Body>) -> crate::Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: hyper::Response<hyper::Body>) -> crate::Result<Self> {
        let (_, body) = response.into_parts();
        let de = &mut serde_json::Deserializer::from_slice(&body)?;

        return serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu);

        let text = response.text().await.context(crate::error::HttpSnafu)?;

        let de = &mut serde_json::Deserializer::from_str(&text);
        serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu)
    }
}
