use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::{BodyExt, Full};
use tower::{BoxError, Service};

use crate::OctoBody;

/// A browser-compatible HTTP service for Octocrab.
///
/// This service is intended for `wasm32-unknown-unknown` targets where the
/// default hyper client is unavailable. It wraps `reqwest`'s browser client and
/// returns collected response bodies that Octocrab can deserialize.
#[derive(Clone, Debug, Default)]
pub struct ReqwestService {
    client: reqwest::Client,
}

impl ReqwestService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Service<Request<OctoBody>> for ReqwestService {
    type Response = Response<Full<Bytes>>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<OctoBody>) -> Self::Future {
        let client = self.client.clone();

        Box::pin(async move {
            let (parts, body) = request.into_parts();
            let body = body.collect().await?.to_bytes();

            let response = client
                .request(parts.method, parts.uri.to_string())
                .headers(parts.headers)
                .body(body)
                .send()
                .await?;

            let status = response.status();
            let headers = response.headers().clone();
            let body = response.bytes().await?;

            let mut builder = Response::builder().status(status);
            *builder.headers_mut().expect("response builder is valid") = headers;

            Ok(builder.body(Full::new(body))?)
        })
    }
}
