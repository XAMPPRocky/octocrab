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

pub mod actions {
    //! Parameter types for the actions API.

    /// The archive format for artifacts.
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum ArchiveFormat {
        Zip,
    }

    impl std::fmt::Display for ArchiveFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let text = match self {
                Self::Zip => "zip",
            };

            f.write_str(text)
        }
    }

    /// Configures the access that repositories have to the organization secret.
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Visibility {
        All,
        Private,
        Selected,
    }
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

pub mod markdown {
    /// The rendering mode.
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    #[non_exhaustive]
    pub enum Mode {
        /// Render a document in plain Markdown, just like README.md files
        /// are rendered.
        Markdown,
        /// Render a document in [GitHub Flavored Markdown][gfm], which creates
        /// links for user mentions as well as references to SHA-1 hashes,
        /// issues, and pull requests.
        ///
        /// [gfm]: https://github.github.com/gfm/
        Gfm,
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

    /// Custom media types are used in the API to let consumers choose the
    /// format of the data they wish to receive. This is done by adding one or
    /// more of the following types to the Accept header when you make a
    /// request. Media types are specific to resources, allowing them to change
    /// independently and support formats that other resources don't.
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    #[non_exhaustive]
    pub enum MediaType {
        Raw,
        Text,
        Html,
        Full,
    }

    impl std::fmt::Display for MediaType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let text = match self {
                Self::Raw => "raw",
                Self::Text => "text",
                Self::Html => "html",
                Self::Full => "full",
            };

            f.write_str(text)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum MergeMethod {
        Merge,
        Squash,
        Rebase,
    }

    pub mod comment {
        /// What to sort results by. Can be either `created` or `updated`.
        #[derive(Debug, Clone, Copy, serde::Serialize)]
        #[serde(rename_all = "snake_case")]
        #[non_exhaustive]
        pub enum Sort {
            Created,
            Updated,
        }
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

    /// A Git reference, either a branch, tag, or rev.
    #[derive(Debug, Clone)]
    pub enum Reference {
        Branch(String),
        Tag(String),
        Commit(String),
    }

    impl Reference {
        pub fn ref_url(&self) -> String {
            match self {
                Self::Branch(branch) => format!("heads/{}", branch),
                Self::Tag(tag) => format!("tags/{}", tag),
                Self::Commit(sha) => sha.clone(),
            }
        }

        pub fn full_ref_url(&self) -> String {
            match self {
                Self::Branch(_) | Self::Tag(_) => format!("refs/{}", self.ref_url()),
                Self::Commit(sha) => sha.clone(),
            }
        }
    }

    impl std::fmt::Display for Reference {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str(&self.full_ref_url())
        }
    }
    pub mod forks {
        /// The available methods to sort repository forks by.
        #[derive(Debug, Clone, Copy, serde::Serialize)]
        #[serde(rename_all = "snake_case")]
        #[non_exhaustive]
        pub enum Sort {
            Newest,
            Oldest,
            Stargazers,
        }
    }
}

pub mod teams {
    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Privacy {
        Secret,
        Closed,
    }

    #[derive(Debug, Clone, Copy, serde::Serialize)]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum Permission {
        Pull,
        Push,
        Admin,
        Maintain,
        Triage,
    }
}
