//! # Common GitHub Parameter Types

/// The status of a issue or pull request.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
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
pub enum Direction {
    #[serde(rename="asc")]
    Ascending,
    #[serde(rename="desc")]
    Descending,
}
