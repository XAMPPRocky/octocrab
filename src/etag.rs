//! Types for handling etags.
pub use hyperx::header::EntityTag;

/// Represents resources identified by etags.
#[derive(Debug, PartialEq)]
pub struct Etagged<T> {
    /// A possible etag.
    ///
    /// It is possible, although unlikely, that a response which should contain an etag header does
    /// not, or that etag header is invalid. In such cases this field will be `None`.
    pub etag: Option<EntityTag>,
    /// The value identified by this etag.
    ///
    /// This can be `None` if we have already received the data which this etag identifies.
    pub value: Option<T>,
}
