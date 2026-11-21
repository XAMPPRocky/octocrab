//! Serialization and Deserialization of timestamps in Github API
//!
//! GitHub API can give (from past experience) either:
//! - a seconds timestamp relative to Epoch, or
//! - a string containing the timestamp in [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format.
//!
//! This module handles transparently both formats to deserialize to [`jiff::Timestamp`].

use std::convert::TryFrom;

use jiff::Timestamp;
use serde::{de, Deserialize};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserializer.deserialize_any(GithubTimestampVisitor)
}

/// Helper struct to tell serde the deserializer to use when working with Option<Timestamp>
#[derive(Debug, Deserialize)]
struct WrappedGithubTimestamp(#[serde(deserialize_with = "deserialize")] Timestamp);

pub(crate) fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<WrappedGithubTimestamp>::deserialize(deserializer)
        .map(|opt_wrapped| opt_wrapped.map(|wrapped| wrapped.0))
}

struct GithubTimestampVisitor;

impl de::Visitor<'_> for GithubTimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("a RFC3339 date and time _string_ or a unix timestamp _integer_ in seconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Timestamp::from_second(v).map_err(E::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        i64::try_from(v)
            .map_err(|_| E::custom(format!("value is not a legal timestamp: {v}")))
            .and_then(|v| Timestamp::from_second(v).map_err(E::custom))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TsWrapper {
        #[serde(deserialize_with = "deserialize")]
        ts: Timestamp,
    }

    #[derive(Debug, Deserialize)]
    struct OptTsWrapper {
        #[serde(deserialize_with = "deserialize_opt")]
        ts: Option<Timestamp>,
    }

    #[test]
    fn deserializes_rfc3339_and_unix_timestamps() {
        let expected: Timestamp = "2023-08-18T13:28:04Z".parse().unwrap();

        let from_z: TsWrapper = serde_json::from_str(r#"{"ts":"2023-08-18T13:28:04Z"}"#).unwrap();
        let from_offset: TsWrapper =
            serde_json::from_str(r#"{"ts":"2023-08-18T15:28:04+02:00"}"#).unwrap();
        let from_unix: TsWrapper = serde_json::from_str(r#"{"ts":1692365284}"#).unwrap();
        let from_neg: TsWrapper = serde_json::from_str(r#"{"ts":-10}"#).unwrap();
        let opt_null: OptTsWrapper = serde_json::from_str(r#"{"ts":null}"#).unwrap();

        assert_eq!(from_z.ts, expected);
        assert_eq!(from_offset.ts, expected);
        assert_eq!(from_unix.ts, expected);
        assert_eq!(from_neg.ts, Timestamp::from_second(-10).unwrap());
        assert_eq!(opt_null.ts, None);

        let too_large = format!(r#"{{"ts":{}}}"#, u64::MAX);
        assert!(serde_json::from_str::<TsWrapper>(&too_large).is_err());
    }
}
