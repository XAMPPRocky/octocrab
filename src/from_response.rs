use snafu::ResultExt;

/// A trait for mapping from a `reqwest::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self> {
        let text = response.text().await.context(crate::error::Http)?;

        serde_json::from_str(&text).with_context(|| crate::error::Json {
            json: serde_json::from_str::<serde_json::Value>(&text).unwrap(),
        })
    }
}
