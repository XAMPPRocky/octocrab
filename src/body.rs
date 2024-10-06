use http_body_util::BodyExt;

use bytes::Bytes;
use http_body::Frame;
use snafu::{Backtrace, GenerateImplicitData};
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, crate::Error>;
type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn boxed<B>(body: B) -> BoxBody
where
    B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    B::Error: Into<BoxError>,
{
    try_downcast(body).unwrap_or_else(|body| {
        body.map_err(|e| crate::Error::Other {
            source: e.into(),
            backtrace: Backtrace::capture(),
        })
        .boxed()
    })
}

fn try_downcast<T, K>(k: K) -> Result<T, K>
where
    T: 'static,
    K: Send + 'static,
{
    let mut k = Some(k);
    if let Some(k) = <dyn std::any::Any>::downcast_mut::<Option<T>>(&mut k) {
        Ok(k.take().unwrap())
    } else {
        Err(k.unwrap())
    }
}

// Define octocrab Body
#[derive(Debug)]
pub struct OctoBody(Arc<RwLock<BoxBody>>);

impl OctoBody {
    /// Create a new `Body` that wraps another [`http_body::Body`].
    pub fn new<B>(body: B) -> Self
    where
        B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
        B::Error: Into<BoxError>,
    {
        try_downcast(body).unwrap_or_else(|body| Self(Arc::new(RwLock::new(boxed(body)))))
    }
    /// Create an empty body.
    pub fn empty() -> Self {
        Self::new(http_body_util::Empty::new())
    }
}

impl Default for OctoBody {
    fn default() -> Self {
        Self::empty()
    }
}

// Implement standard Bodiesque casting
impl From<()> for OctoBody {
    fn from(_: ()) -> Self {
        Self::empty()
    }
}

impl From<String> for OctoBody {
    fn from(buf: String) -> Self {
        Self::new(http_body_util::Full::from(buf))
    }
}

impl From<Vec<u8>> for OctoBody {
    fn from(buf: Vec<u8>) -> Self {
        Self::new(http_body_util::Full::from(buf))
    }
}

impl From<Bytes> for OctoBody {
    fn from(buf: Bytes) -> Self {
        Self::new(http_body_util::Full::from(buf))
    }
}

impl From<&'static str> for OctoBody {
    fn from(buf: &'static str) -> Self {
        Self::new(http_body_util::Full::from(buf))
    }
}

impl http_body::Body for OctoBody {
    type Data = Bytes;
    type Error = crate::Error;

    #[inline]
    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let inner = Pin::into_inner(self);
        let mut boxed_body = inner.0.write().expect("RwLock write lock failed");
        Pin::new(&mut *boxed_body).poll_frame(cx)
    }

    #[inline]
    fn size_hint(&self) -> http_body::SizeHint {
        let b = self.0.read().expect("RwLock read lock failed");
        b.size_hint()
    }

    #[inline]
    fn is_end_stream(&self) -> bool {
        let b = self.0.read().expect("RwLock read lock failed");
        b.is_end_stream()
    }
}

impl Clone for OctoBody {
    fn clone(&self) -> Self {
        OctoBody(Arc::clone(&self.0))
    }
}
