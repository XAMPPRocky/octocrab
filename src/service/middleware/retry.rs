use futures_util::future;
use http::{Request, Response};
use hyper_util::client::legacy::Error;
use tower::retry::Policy;

#[derive(Clone)]
pub enum RetryConfig {
    None,
    Simple(usize),
}

impl<B> Policy<Request<String>, Response<B>, Error> for RetryConfig {
    type Future = futures_util::future::Ready<Self>;

    fn retry(
        &self,
        _req: &Request<String>,
        result: Result<&Response<B>, &Error>,
    ) -> Option<Self::Future> {
        match self {
            RetryConfig::None => None,
            RetryConfig::Simple(count) => match result {
                Ok(response) => {
                    if response.status().is_server_error() || response.status() == 429 {
                        if *count > 0 {
                            Some(future::ready(RetryConfig::Simple(count - 1)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => {
                    if *count > 0 {
                        Some(future::ready(RetryConfig::Simple(count - 1)))
                    } else {
                        None
                    }
                }
            },
        }
    }

    fn clone_request(&self, req: &Request<String>) -> Option<Request<String>> {
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
