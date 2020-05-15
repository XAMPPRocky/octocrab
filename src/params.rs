//! # Common GitHub Parameter Types

/// The status of a issue or pull request.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum State {
    All,
    Open,
    Closed,
}

/// What to sort results by. Can be either `created`, `updated`, `popularity`
/// (comment count) or `long-running` (age, filtering by pulls updated in the
/// last month).
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Direction {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// The reason for locking an issue.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[non_exhaustive]
pub enum LockReason {
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "spam")]
    Spam,
}

pub mod issues {
    //! Parameter types for the issues API.

    /// What to sort the results by. Can be either `created`, `updated`,
    /// or `comments`.
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Sort {
        Created,
        Updated,
        Comments,
    }

    /// A generic filter type that allows you to filter either by exact match,
    /// any match, or no matches.
    #[derive(Debug, Clone, Copy)]
    #[non_exhaustive]
    pub enum Filter<T> {
        Matches(T),
        Any,
        None,
    }

    impl<T: serde::Serialize> serde::Serialize for Filter<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Self::Matches(val) => val.serialize(serializer),
                Self::Any => serializer.serialize_str("*"),
                Self::None => serializer.serialize_str("none"),
            }
        }
    }

    impl<T: serde::Serialize> From<T> for Filter<T> {
        fn from(value: T) -> Self {
            Self::Matches(value)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(
                "1234",
                serde_json::to_string(&Filter::Matches(1234)).unwrap()
            );
            assert_eq!(
                r#""milestone""#,
                serde_json::to_string(&Filter::Matches("milestone")).unwrap()
            );
            assert_eq!(r#""*""#, serde_json::to_string(&Filter::<()>::Any).unwrap());
            assert_eq!(
                r#""none""#,
                serde_json::to_string(&Filter::<()>::None).unwrap()
            );
        }
    }
}

pub mod orgs {
    //! Parameter types for the organization API.

    /// What to sort results by. Can be either `created`, `updated`, `popularity`
    /// (comment count) or `long-running` (age, filtering by pulls updated in the
    /// last month).
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Role {
        Member,
        Admin,
    }
}

pub mod pulls {
    //! Parameter types for the pull request API.

    /// What to sort results by. Can be either `created`, `updated`, `popularity`
    /// (comment count) or `long-running` (age, filtering by pulls updated in the
    /// last month).
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Sort {
        Created,
        Updated,
        Popularity,
        LongRunning,
    }
}

pub mod repos {
    /// The type of repository to search for.
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Type {
        /// All repositories, usually the default.
        All,
        /// All forked rpositories.
        Forks,
        /// Only available if your organization is associated with an enterprise
        /// account using GitHub Enterprise Cloud or GitHub Enterprise
        /// Server 2.20+.
        Internal,
        /// All member repositories
        Member,
        ///  All private repositores
        Private,
        /// All public repositories
        Public,
        /// All source repostories (a repository that is not a fork).
        Sources,
    }

    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Sort {
        Created,
        Updated,
        Pushed,
        FullName,
    }
}
