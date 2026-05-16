use bytes::Bytes;
use http_body::Body;
use http_body_util::BodyExt;
use snafu::ResultExt;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// A trait for mapping from a `http::Response` to an another type.
        #[async_trait::async_trait(?Send)]
        pub trait FromResponse: Sized {
            async fn from_response<B>(response: http::Response<B>) -> crate::Result<Self>
            where
                B: Body<Data = Bytes, Error = crate::Error>;
        }
    } else {
        /// A trait for mapping from a `http::Response` to an another type.
        #[async_trait::async_trait]
        pub trait FromResponse: Sized {
            async fn from_response<B>(response: http::Response<B>) -> crate::Result<Self>
            where
                B: Body<Data = Bytes, Error = crate::Error> + Send;
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[async_trait::async_trait(?Send)]
        impl<T: serde::de::DeserializeOwned> FromResponse for T {
            async fn from_response<B>(response: http::Response<B>) -> crate::Result<Self>
            where
                B: Body<Data = Bytes, Error = crate::Error>,
            {
                let (_, body) = response.into_parts();
                let body = body.collect().await?.to_bytes();
                let de = &mut serde_json::Deserializer::from_slice(&body);
                return serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu);
            }
        }
    } else {
        #[async_trait::async_trait]
        impl<T: serde::de::DeserializeOwned> FromResponse for T {
            async fn from_response<B>(response: http::Response<B>) -> crate::Result<Self>
            where
                B: Body<Data = Bytes, Error = crate::Error> + Send,
            {
                let (_, body) = response.into_parts();
                let body = body.collect().await?.to_bytes();
                let de = &mut serde_json::Deserializer::from_slice(&body);
                return serde_path_to_error::deserialize(de).context(crate::error::JsonSnafu);
            }
        }
    }
}
