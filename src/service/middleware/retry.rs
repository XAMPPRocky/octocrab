use std::time::{Duration, SystemTime, UNIX_EPOCH};

use http::{HeaderMap, Request, Response, StatusCode};
use hyper::{Body, Error};
use tower::retry::Policy;

/// The factor by which to increase the fallback retry delay. This is only
/// used when the fallback delay is needed (ie. when a delay couldn't be found
/// in the response).
const RETRY_SCALE_FACTOR: f64 = 2.0;

#[derive(Clone)]
pub enum RetryConfig {
    None,
    Simple(usize),
    /// Retries a request factoring in delays specified in response headers.
    /// If none can be found, falls back to exponential backoff.
    /// https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#rate-limiting
    ResponseOrExponentialBackoff {
        /// The delay to use if no delay can be determined from the response.
        fallback_delay: Duration,
        /// The max delay to allow in any situation. Github resets rate limits
        /// hourly, so that's effectively the worst case delay. Instead of
        /// waiting that long, opt to fail faster by capping delays to this
        /// value.
        max_delay: Duration,
        /// Maximum attempts to make for a request.
        count: usize,
    },
}

impl Policy<Request<String>, Response<hyper::Body>, hyper::Error> for RetryConfig {
    type Future = tokio::time::Sleep;

    fn retry(
        &mut self,
        _req: &mut Request<std::string::String>,
        result: &mut Result<Response<Body>, Error>,
    ) -> Option<Self::Future> {
        let mut delay = Duration::ZERO;
        match self {
            RetryConfig::None => {
                return None;
            }
            RetryConfig::Simple(count) => {
                if *count <= 0 {
                    return None;
                }
                if let Ok(response) = result {
                    let status = response.status();
                    // Oddly, Github sends back 403 status codes for rate limit
                    // errors. The presence of rate limit headers will help us
                    // distinguish between rate limit errors vs. other bad
                    // requests.
                    if !(status.is_server_error()
                        || status == StatusCode::TOO_MANY_REQUESTS
                        || status == StatusCode::BAD_REQUEST)
                    {
                        return None;
                    }
                    if status == StatusCode::BAD_REQUEST {
                        if let None = determine_next_delay(response.headers()) {
                            return None;
                        }
                    }
                }
                *count -= 1;
            }
            RetryConfig::ResponseOrExponentialBackoff {
                fallback_delay,
                max_delay,
                count,
            } => {
                if *count <= 0 {
                    return None;
                }
                let mut next_fallback_delay = *fallback_delay;
                match result {
                    Ok(response) => {
                        let status = response.status();
                        // Oddly, Github sends back 403 status codes for rate limit
                        // errors. The presence of rate limit headers will help us
                        // distinguish between rate limit errors vs. other bad
                        // requests.
                        if !(status.is_server_error()
                            || status == StatusCode::TOO_MANY_REQUESTS
                            || status == StatusCode::BAD_REQUEST)
                        {
                            return None;
                        }
                        let delay_response = determine_next_delay(response.headers());
                        if status == StatusCode::BAD_REQUEST && delay_response.is_none() {
                            return None;
                        }
                        delay = match delay_response {
                            Some(d) => Duration::from_secs_f64(d),
                            None => {
                                next_fallback_delay =
                                    next_fallback_delay.mul_f64(RETRY_SCALE_FACTOR);
                                *fallback_delay
                            }
                        };
                    }
                    Err(_) => {
                        next_fallback_delay = next_fallback_delay.mul_f64(RETRY_SCALE_FACTOR);
                        delay = *fallback_delay;
                    }
                }
                delay = delay.min(*max_delay);
                *fallback_delay = next_fallback_delay;
                *count -= 1;
            }
        }
        Some(tokio::time::sleep(delay))
    }

    fn clone_request(&mut self, req: &Request<String>) -> Option<Request<String>> {
        match self {
            RetryConfig::None => None,
            _ => {
                // `Request` can't be cloned
                let mut new_req = Request::builder()
                    .uri(req.uri())
                    .method(req.method())
                    .version(req.version());
                for (name, value) in req.headers() {
                    new_req = new_req.header(name, value);
                }

                let body = req.body().clone();
                let new_req = new_req.body(body).expect(
                    "This should never panic, as we are cloning a components from existing request",
                );

                Some(new_req)
            }
        }
    }
}

fn determine_next_delay(headers: &HeaderMap) -> Option<f64> {
    // retry-after is a duration
    if let Some(retry_after) = headers.get("retry-after") {
        retry_after
            .to_str()
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
    } else if let Some(reset) = headers.get("x-ratelimit-reset") {
        // x-ratelimit-reset is the Unix timestamp when the rate limit will reset.
        reset
            .to_str()
            .ok()
            .and_then(|s| s.parse().ok())
            .and_then(|f| {
                (UNIX_EPOCH + Duration::from_secs_f64(f))
                    .duration_since(SystemTime::now())
                    .ok()
                    .and_then(|d| Some(d.as_secs_f64()))
            })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use http::{Request, Response, StatusCode};
    use tower::retry::Policy;

    use super::RetryConfig;

    fn simple_retry_when_no_attempts_left() {
        let mut retry = RetryConfig::Simple(0);
        let mut resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_none());
        assert!(matches!(retry, RetryConfig::Simple(0)));
    }

    fn simple_retry_when_attempts_left() {
        let mut retry = RetryConfig::Simple(3);
        let mut resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_some());
        assert!(matches!(retry, RetryConfig::Simple(2)));

        let mut retry = RetryConfig::Simple(3);
        resp = Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_some());
        assert!(matches!(retry, RetryConfig::Simple(2)));

        let mut retry = RetryConfig::Simple(3);
        resp = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("retry-after", "1")
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_some());
        assert!(matches!(retry, RetryConfig::Simple(2)));
    }

    fn response_or_backoff_when_no_attempts_left() {
        let mut retry = RetryConfig::ResponseOrExponentialBackoff {
            fallback_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            count: 0,
        };
        let mut resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_none());
        assert!(matches!(
            retry,
            RetryConfig::ResponseOrExponentialBackoff {
                fallback_delay: Duration::ZERO,
                max_delay: Duration::ZERO,
                count: 0,
            }
        ));
    }

    fn response_or_backoff_when_attempts_left_using_fallbacks() {
        let mut retry = RetryConfig::ResponseOrExponentialBackoff {
            fallback_delay: Duration::from_micros(1),
            max_delay: Duration::from_micros(10),
            count: 3,
        };
        let mut resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_some());
        let expect_fallback: Duration = Duration::from_micros(2);
        assert!(matches!(
            retry,
            RetryConfig::ResponseOrExponentialBackoff {
                fallback_delay: expect_fallback,
                count: 2,
                ..
            }
        ));
    }

    fn response_or_backoff_when_attempts_left_using_fallbacks_capped_delay() {
        let mut retry = RetryConfig::ResponseOrExponentialBackoff {
            fallback_delay: Duration::from_micros(20),
            max_delay: Duration::from_micros(10),
            count: 3,
        };
        let mut resp = Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_some());
        let expect_fallback = Duration::from_micros(10);
        assert!(matches!(
            retry,
            RetryConfig::ResponseOrExponentialBackoff {
                fallback_delay: expect_fallback,
                count: 2,
                ..
            }
        ));
    }

    fn response_or_backoff_when_attempts_left_using_response_headers() {
        let mut retry = RetryConfig::ResponseOrExponentialBackoff {
            fallback_delay: Duration::from_micros(1),
            max_delay: Duration::from_micros(10),
            count: 3,
        };
        let mut resp = Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("retry-after", "1")
            .body("".into())
            .unwrap();
        let next = retry.retry(&mut Request::new("".into()), &mut Ok(resp));
        assert!(next.is_some());
        let expect_fallback = Duration::from_micros(1);
        assert!(matches!(
            retry,
            RetryConfig::ResponseOrExponentialBackoff {
                fallback_delay: expect_fallback,
                count: 2,
                ..
            }
        ));
    }

    fn determine_rate_limit_when_no_delay_is_given() {
        let headers = http::HeaderMap::new();
        let delay = super::determine_next_delay(&headers);
        assert_eq!(delay, None);
    }

    fn determine_rate_limit_when_retry_after_is_given() {
        let mut headers = http::HeaderMap::new();
        headers.insert("retry-after", "1".parse().unwrap());
        let delay = super::determine_next_delay(&headers);
        assert_eq!(delay, Some(1.0));
    }

    fn determine_rate_limit_when_retry_after_is_invalid() {
        let mut headers = http::HeaderMap::new();
        headers.insert("retry-after", "invalid".parse().unwrap());
        let delay = super::determine_next_delay(&headers);
        assert_eq!(delay, None);
    }

    fn determine_rate_limit_when_rate_limit_reset_is_given() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut headers = http::HeaderMap::new();
        let d = (SystemTime::now() + Duration::from_secs(2))
            .duration_since(UNIX_EPOCH)
            .unwrap();
        headers.insert(
            "x-ratelimit-reset",
            d.as_secs().to_string().parse().unwrap(),
        );
        let delay = super::determine_next_delay(&headers);
        assert_eq!(delay, Some(2.0));
    }

    fn determine_rate_limit_when_rate_limit_reset_is_invalid() {
        let mut headers = http::HeaderMap::new();
        headers.insert("x-ratelimit-reset", "invalid".parse().unwrap());
        let delay = super::determine_next_delay(&headers);
        assert_eq!(delay, None);
    }
}
