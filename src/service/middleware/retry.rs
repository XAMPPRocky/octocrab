use http::{Request, Response};
use hyper_util::client::legacy::Error;
use tower::retry::Policy;

use crate::body::OctoBody;

#[derive(Clone)]
pub enum RetryConfig {
    None,
    Simple(usize),
}

impl<B> Policy<Request<OctoBody>, Response<B>, Error> for RetryConfig {
    type Future = futures_util::future::Ready<Self>;

    fn retry(
        &self,
        _req: &Request<OctoBody>,
        _result: Result<&Response<B>, &Error>,
    ) -> Option<Self::Future> {
        match self {
            RetryConfig::None => None,
            RetryConfig::Simple(_count) => None,
        }
    }

    fn clone_request(&self, _req: &Request<OctoBody>) -> Option<Request<OctoBody>> {
        match self {
            RetryConfig::None => None,
            _ => None,
        }
    }
}
