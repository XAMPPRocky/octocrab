use snafu::ResultExt;

#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self> {
        response.json().await.context(crate::error::Http)
    }
}
