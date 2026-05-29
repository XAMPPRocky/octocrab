use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::{BodyExt, Full};
use snafu::{Backtrace, ResultExt};
use tower::Service;

use crate::body::OctoBody;
use crate::error;
use crate::Error;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Error>;

#[derive(Clone, Debug)]
pub struct WasmClient {
    client: reqwest::Client,
}

impl WasmClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Service<Request<OctoBody>> for WasmClient {
    type Response = Response<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<OctoBody>) -> Self::Future {
        let client = self.client.clone();

        Box::pin(async move {
            let (parts, body) = request.into_parts();
            let body = body.collect().await?.to_bytes();

            let mut request = client
                .request(parts.method, parts.uri.to_string())
                .body(body);

            for (name, value) in parts.headers {
                if let Some(name) = name {
                    request = request.header(name.as_str(), value.as_bytes());
                }
            }

            let response = request.send().await.map_err(|source| Error::Other {
                source: Box::new(source),
                backtrace: Backtrace::capture(),
            })?;

            let mut builder = Response::builder().status(response.status());

            for (name, value) in response.headers() {
                builder = builder.header(name.as_str(), value.as_bytes());
            }

            let body = response.bytes().await.map_err(|source| Error::Other {
                source: Box::new(source),
                backtrace: Backtrace::capture(),
            })?;

            builder
                .body(Full::from(body).map_err(|never| match never {}).boxed())
                .context(error::HttpSnafu)
        })
    }
}
