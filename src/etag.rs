//! Types for handling etags.
use reqwest::header::HeaderValue;
use snafu::ResultExt;
use std::convert::TryFrom;

/// Represents resources identified by etags.
#[derive(Debug, PartialEq)]
pub struct Etagged<T> {
    /// A possible etag.
    ///
    /// It is possible, although unlikely, that a response which should contain an etag header does
    /// not, or that etag header is invalid. In such cases this field will be `None`.
    pub etag: Option<Etag>,
    /// The value identified by this etag.
    ///
    /// This can be `None` if we have already received the data which this etag identifies.
    pub value: Option<T>,
}

/// An etag.
#[derive(Debug, PartialEq)]
pub struct Etag(String);

impl Etag {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl TryFrom<&str> for Etag {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_ascii() {
            Ok(Etag(value.to_owned()))
        } else {
            Err("An Etag can only contain ASCII characters".into()).context(crate::error::Other)
        }
    }
}

impl TryFrom<HeaderValue> for Etag {
    type Error = crate::Error;

    fn try_from(value: HeaderValue) -> Result<Self, Self::Error> {
        convert_header_value(&value)
    }
}

impl TryFrom<&HeaderValue> for Etag {
    type Error = crate::Error;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        convert_header_value(value)
    }
}

fn convert_header_value(value: &HeaderValue) -> crate::Result<Etag> {
    value
        .to_str()
        .map(|val| Etag(val.to_owned()))
        .map_err(|_| "Received etag header containing non-ASCII characters".into())
        .context(crate::error::Other)
}

impl From<Etag> for HeaderValue {
    fn from(etag: Etag) -> Self {
        // By construction, the Etag is guaranteed to have only ASCII (non-extended) characters, so
        // unwrapping here is ok.
        HeaderValue::from_str(etag.as_str()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Etag;
    use reqwest::header::HeaderValue;
    use std::convert::TryFrom;

    #[test]
    fn str_slice_to_etag_conversion_should_succeed_if_ascii() {
        let etag_str = "1234";
        let result = Etag::try_from(etag_str);
        assert!(result.is_ok(), "unexpected error: {:#?}", result);
        assert_eq!(result.ok().unwrap().as_str(), etag_str);
    }

    #[test]
    fn str_slice_to_etag_conversion_should_fail_if_non_ascii() {
        let etag_str = "123µ";
        let result = Etag::try_from(etag_str);
        assert!(
            result.is_err(),
            "expected an error, instead it returned: {:#?}",
            result
        );
        let expected_msg = "An Etag can only contain ASCII characters";
        let error = result.err().unwrap().to_string();
        assert!(
            error.contains(expected_msg),
            "error '{}' did not contain expected message",
            error
        );
    }

    #[test]
    fn header_value_ref_to_etag_should_fail_with_non_ascii() {
        let value = HeaderValue::from_bytes(b"123\xb5").unwrap();
        let result = Etag::try_from(&value);
        assert!(
            result.is_err(),
            "expected unsuccessful conversion, got etag {:?}",
            result
        );
        let error = result.err().unwrap();
        assert!(
            error
                .to_string()
                .contains("Received etag header containing non-ASCII characters"),
            "expected error to contain specific string, got: {:?}",
            error
        );
    }

    #[test]
    fn header_value_to_etag_should_fail_with_non_ascii() {
        let value = HeaderValue::from_bytes(b"123\xb5").unwrap();
        let result = Etag::try_from(value);
        assert!(
            result.is_err(),
            "expected unsuccessful conversion, got etag {:?}",
            result
        );
        let error = result.err().unwrap();
        assert!(
            error
                .to_string()
                .contains("Received etag header containing non-ASCII characters"),
            "expected error to contain specific string, got: {:?}",
            error
        );
    }
}
