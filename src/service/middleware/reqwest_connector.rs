//! A [`tower::Service`] adapter that executes requests using a
//! [`reqwest::Client`].
//!
//! This allows Octocrab's existing Tower middleware stack (retries, tracing,
//! auth headers, base URI rewriting, HTTP caching, etc., see
//! [`crate::OctocrabBuilder::build`]) to run on top of `reqwest`'s HTTP
//! engine instead of the default `hyper`-based one. In particular this gives
//! access to `reqwest`'s mature proxy support (including honoring the
//! `HTTP_PROXY`/`HTTPS_PROXY`/`NO_PROXY` environment variables, as well as
//! explicit [`reqwest::Proxy`] configuration) and its TLS backend selection.
//!
//! This service is purely additive: it does not change any of Octocrab's
//! existing behavior, and is only used when a caller opts in via
//! [`crate::OctocrabBuilder::build_with_reqwest`].
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::{combinators::BoxBody, BodyExt};
use snafu::ResultExt;
use tower::Service;

use crate::body::OctoBody;
use crate::error::ReqwestSnafu;

type ResBody = BoxBody<Bytes, crate::Error>;

/// A [`tower::Service`] that sends requests through a [`reqwest::Client`].
///
/// This is meant to be used as the innermost ("leaf") service of Octocrab's
/// Tower stack, in place of the default `hyper`-based connector. It performs
/// no middleware behavior of its own; it only converts between
/// Octocrab/`http` types and `reqwest` types.
///
/// Create an [`Octocrab`](crate::Octocrab) instance backed by this service
/// with [`OctocrabBuilder::build_with_reqwest`](crate::OctocrabBuilder::build_with_reqwest)
/// or [`OctocrabBuilder::build_with_reqwest_client`](crate::OctocrabBuilder::build_with_reqwest_client).
#[derive(Clone, Debug)]
pub struct ReqwestConnector {
    client: reqwest::Client,
}

impl ReqwestConnector {
    /// Wrap an existing [`reqwest::Client`] as a [`tower::Service`].
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Service<Request<OctoBody>> for ReqwestConnector {
    type Response = Response<ResBody>;
    type Error = crate::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // `reqwest::Client` manages its own connection pool and readiness
        // internally, so this service is always ready to accept work.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<OctoBody>) -> Self::Future {
        let client = self.client.clone();

        Box::pin(async move {
            let (parts, body) = req.into_parts();
            // `reqwest::Body::wrap` streams the body straight through rather
            // than buffering it, since `OctoBody` already implements
            // `http_body::Body`.
            let body = reqwest::Body::wrap(body);
            let req = Request::from_parts(parts, body);

            let req = reqwest::Request::try_from(req).context(ReqwestSnafu)?;
            let res = client.execute(req).await.context(ReqwestSnafu)?;

            let res: Response<reqwest::Body> = res.into();
            let (parts, body) = res.into_parts();
            let body: ResBody = body
                .map_err(|source| crate::Error::Reqwest {
                    source,
                    backtrace: snafu::Backtrace::capture(),
                })
                .boxed();

            Ok(Response::from_parts(parts, body))
        })
    }
}
