use futures_util::{future, FutureExt};
use http::header::AsHeaderName;
use http::{HeaderMap, HeaderValue, Request, Response};
use std::sync::Arc;
use std::time::Duration;
use tower::retry::Policy;

use crate::body::OctoBody;

fn header_as_u64(headers: &HeaderMap<HeaderValue>, header: impl AsHeaderName) -> Option<u64> {
    headers.get(header)?.to_str().ok()?.parse().ok()
}
fn header_as_i64(headers: &HeaderMap<HeaderValue>, header: impl AsHeaderName) -> Option<i64> {
    headers.get(header)?.to_str().ok()?.parse().ok()
}

/// Gather metrics about retry behavior when handling rate limit headers.
pub trait RateLimitMetrics: Send + Sync {
    /// An error occurred and either was not a 403/429, or did not have any rate limit headers
    fn retry_after_error(
        &self,
        req: &Request<OctoBody>,
        status_code: http::StatusCode,
        retries_remaining: usize,
    );
    /// A 403/429 error occurred, and rate limit headers were available.
    ///
    /// The handler will wait for `waiting_seconds` before retrying.
    fn rate_limited(
        &self,
        req: &Request<OctoBody>,
        status_code: http::StatusCode,
        retries_remaining: usize,
        waiting_seconds: u64,
    );
}

/// Simple No-op struct for users who do not care about collecting retry metrics
pub struct NoOpRateLimitMetrics;
impl RateLimitMetrics for NoOpRateLimitMetrics {
    fn retry_after_error(
        &self,
        _url: &Request<OctoBody>,
        _status_code: http::StatusCode,
        _retries_remaining: usize,
    ) {
    }
    fn rate_limited(
        &self,
        _url: &Request<OctoBody>,
        _status_code: http::StatusCode,
        _retries_remaining: usize,
        _waiting_seconds: u64,
    ) {
    }
}

#[derive(Clone)]
pub enum RetryConfig {
    None,
    Simple(usize),
    /// Handle GitHub's retry headers and transport errors, up to `max_retries` times.
    ///
    /// Per the rate limit documentation here: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
    /// - If we get a 403/429 and can parse the headers, wait until the refresh period before retrying.
    /// - If we get a 429 and none of the headers are present, wait `min_wait_seconds` seconds.
    /// - If we get a 403 and none of the headers are present, wait `min_wait_seconds` seconds and
    ///   retry when `retry_on_forbidden` is `true`. When it is `false`, do not retry because it is
    ///   not clear whether the response is actually forbidden or is a rate limit.
    /// - For server errors (5xx), retry immediately
    /// - For transport errors that occur before receiving an HTTP response, retry immediately.
    /// - For any other errors do not retry.
    HandleRateLimits {
        metrics: Arc<dyn RateLimitMetrics>,
        max_retries: usize,
        min_wait_seconds: u64,
        /// When `true`, a `403` response without a `retry-after` header and without
        /// `x-ratelimit-remaining: 0` is treated as a secondary rate limit: the handler waits
        /// `min_wait_seconds` and retries. When `false` (the default-conservative choice), such a
        /// `403` is not retried, because it may be a genuine authorization failure rather than a
        /// rate limit.
        retry_on_forbidden: bool,
    },
}

// Generic over the inner service's error type `E` (not just its response
// body `B`) so that this policy can be reused with any leaf transport
// service, e.g. both the default hyper-based client and the optional
// reqwest-based client (see `crate::service::middleware::reqwest_connector`).
impl<B, E> Policy<Request<OctoBody>, Response<B>, E> for RetryConfig {
    type Future = future::BoxFuture<'static, ()>;

    fn retry(
        &mut self,
        req: &mut Request<OctoBody>,
        result: &mut Result<Response<B>, E>,
    ) -> Option<Self::Future> {
        match self {
            RetryConfig::None => None,
            RetryConfig::Simple(count) => match result {
                Ok(response) => {
                    if response.status().is_server_error() || response.status() == 429 {
                        if *count > 0 {
                            *count -= 1;
                            Some(future::ready(()).boxed())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => {
                    if *count > 0 {
                        *count -= 1;
                        Some(future::ready(()).boxed())
                    } else {
                        None
                    }
                }
            },
            RetryConfig::HandleRateLimits {
                metrics,
                max_retries,
                min_wait_seconds,
                retry_on_forbidden,
            } => {
                if *max_retries > 0 {
                    let response = match result.as_ref() {
                        Ok(response) => response,
                        Err(_) => {
                            *max_retries -= 1;
                            return Some(future::ready(()).boxed());
                        }
                    };

                    if matches!(
                        response.status(),
                        http::StatusCode::TOO_MANY_REQUESTS | http::StatusCode::FORBIDDEN
                    ) {
                        *max_retries -= 1;

                        let headers = response.headers();
                        let wait_secs = match (
                            header_as_u64(headers, "retry-after"),
                            header_as_u64(headers, "x-ratelimit-remaining"),
                            header_as_i64(headers, "x-ratelimit-reset"),
                        ) {
                            (Some(secs), _, _) => Some(secs),
                            (None, Some(0), Some(reset_ts)) => {
                                Some(std::cmp::max(5, reset_ts - chrono::Utc::now().timestamp())
                                    as u64)
                            }
                            (None, _, _)
                                if response.status() == http::StatusCode::TOO_MANY_REQUESTS
                                    || (response.status() == http::StatusCode::FORBIDDEN
                                        && *retry_on_forbidden) =>
                            {
                                Some(*min_wait_seconds)
                            }
                            _ => {
                                metrics.retry_after_error(req, response.status(), *max_retries);
                                None
                            }
                        }?;

                        metrics.rate_limited(req, response.status(), *max_retries, wait_secs);
                        Some(
                            tokio::time::sleep(Duration::from_secs(wait_secs))
                                .then(move |_| future::ready(()))
                                .boxed(),
                        )
                    } else if response.status().is_server_error() {
                        *max_retries -= 1;
                        metrics.retry_after_error(req, response.status(), *max_retries);
                        Some(future::ready(()).boxed())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn clone_request(&mut self, req: &Request<OctoBody>) -> Option<Request<OctoBody>> {
        match self {
            RetryConfig::None => None,
            _ => {
                // This returns none if the body is empty. Just return an empty body
                // instead so that we retry GET requests.
                let body = req.body().try_clone().unwrap_or_else(OctoBody::empty);

                // `Request` can't be cloned
                let mut new_req = Request::builder()
                    .uri(req.uri())
                    .method(req.method())
                    .version(req.version());
                for (name, value) in req.headers() {
                    new_req = new_req.header(name, value);
                }

                let new_req = new_req.body(body).expect(
                    "This should never panic, as we are cloning a components from existing request",
                );
                Some(new_req)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NoOpRateLimitMetrics, RetryConfig};
    use crate::body::OctoBody;
    use http::{Request, Response, StatusCode};
    use std::sync::Arc;
    use tower::retry::Policy;

    fn policy(retry_on_forbidden: bool, max_retries: usize) -> RetryConfig {
        RetryConfig::HandleRateLimits {
            metrics: Arc::new(NoOpRateLimitMetrics),
            max_retries,
            min_wait_seconds: 60,
            retry_on_forbidden,
        }
    }

    fn request() -> Request<OctoBody> {
        Request::builder().body(OctoBody::empty()).unwrap()
    }

    fn response(
        status: StatusCode,
        headers: &[(&'static str, String)],
    ) -> Result<Response<()>, ()> {
        let mut response = Response::builder().status(status);
        for (name, value) in headers {
            response = response.header(*name, value.as_str());
        }
        Ok(response.body(()).unwrap())
    }

    fn will_retry(policy: &mut RetryConfig, result: &mut Result<Response<()>, ()>) -> bool {
        policy.retry(&mut request(), result).is_some()
    }

    #[tokio::test]
    async fn retries_transport_error() {
        let mut policy = policy(false, 1);
        let mut result = Err(());

        assert!(will_retry(&mut policy, &mut result));
        assert!(!will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn retries_forbidden_with_retry_after() {
        let mut policy = policy(false, 3);
        let mut result = response(StatusCode::FORBIDDEN, &[("retry-after", "5".into())]);

        assert!(will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn retries_forbidden_when_primary_rate_limit_is_exhausted() {
        let mut policy = policy(false, 3);
        let reset = (chrono::Utc::now().timestamp() + 60).to_string();
        let mut result = response(
            StatusCode::FORBIDDEN,
            &[
                ("x-ratelimit-remaining", "0".into()),
                ("x-ratelimit-reset", reset),
            ],
        );

        assert!(will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn does_not_retry_bare_forbidden_by_default() {
        let mut policy = policy(false, 3);
        let mut result = response(StatusCode::FORBIDDEN, &[]);

        assert!(!will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn retries_bare_forbidden_when_enabled() {
        let mut policy = policy(true, 3);
        let mut result = response(StatusCode::FORBIDDEN, &[]);

        assert!(will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn retries_bare_too_many_requests_regardless_of_forbidden_setting() {
        for retry_on_forbidden in [false, true] {
            let mut policy = policy(retry_on_forbidden, 3);
            let mut result = response(StatusCode::TOO_MANY_REQUESTS, &[]);

            assert!(will_retry(&mut policy, &mut result));
        }
    }

    #[tokio::test]
    async fn does_not_retry_success() {
        let mut policy = policy(false, 3);
        let mut result = response(StatusCode::OK, &[]);

        assert!(!will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn retries_server_error() {
        let mut policy = policy(false, 3);
        let mut result = response(StatusCode::INTERNAL_SERVER_ERROR, &[]);

        assert!(will_retry(&mut policy, &mut result));
    }

    #[tokio::test]
    async fn does_not_retry_too_many_requests_when_retries_are_exhausted() {
        let mut policy = policy(false, 0);
        let mut result = response(StatusCode::TOO_MANY_REQUESTS, &[]);

        assert!(!will_retry(&mut policy, &mut result));
    }
}
