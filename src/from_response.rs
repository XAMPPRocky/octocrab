use std::env;

use snafu::ResultExt;

/// A trait for mapping from a `reqwest::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self> {
        let text = response.text().await.context(crate::error::HttpSnafu)?;

        let de = &mut serde_json::Deserializer::from_str(&text);
        match serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu) {
            Ok(value) => Ok(value),
            Err(e) => {
                if env::var("DEBUG_OCTOCRAB").eq(&Ok("1".to_string())) {
                    println!("Error from HTTP response: {:#?}", e);
                }
                Err(e)
            }
        }
    }
}
