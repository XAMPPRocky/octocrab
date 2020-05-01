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
