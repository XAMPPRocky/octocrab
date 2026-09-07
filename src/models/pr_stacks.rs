//! Models for GitHub's Stacked Pull Requests API.
//!
//! See <https://docs.github.com/en/rest/pulls/stacks?apiVersion=2022-11-28>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

/// A minimal pull request entry as returned inside a [`PrStack`].
///
/// This is a compact representation — it only contains the fields
/// the Stacks API returns, not the full pull request payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StackedPrEntry {
    /// The pull request number.
    pub number: u64,
    /// Current state of the PR (`"open"` or `"closed"`).
    pub state: String,
    /// Whether this PR is a draft.
    pub draft: bool,
    /// Timestamp when the PR was merged, or `None` if unmerged.
    pub merged_at: Option<DateTime<Utc>>,
    /// Head branch ref and SHA.
    pub head: PrHeadRef,
}

/// Head branch information for a pull request inside a stack.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PrHeadRef {
    /// Branch ref name (e.g. `"feature/my-branch"`).
    #[serde(rename = "ref")]
    pub ref_field: String,
    /// Commit SHA at the head of the branch.
    pub sha: String,
}

/// The base branch a stack targets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StackBase {
    /// Base branch ref name (e.g. `"main"`).
    #[serde(rename = "ref")]
    pub ref_field: String,
}

/// A pull request stack, as returned by the GitHub Stacked Pull Requests API.
///
/// A stack is an ordered sequence of pull requests where each PR's head branch
/// is the base of the next PR in the stack.
///
/// # Example
/// ```no_run
/// # async fn run() -> octocrab::Result<()> {
/// let stacks = octocrab::instance()
///     .repos("owner", "repo")
///     .stacks()
///     .list()
///     .await?;
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PrStack {
    /// Unique database ID for this stack.
    pub id: u64,
    /// Stack number within the repository (used in API paths as `{stack_number}`).
    pub number: u64,
    /// GitHub node ID.
    pub node_id: String,
    /// API URL for this stack resource.
    pub url: Url,
    /// The base branch this stack targets.
    pub base: StackBase,
    /// Whether the stack is open (i.e., contains at least one unmerged PR).
    pub open: bool,
    /// When this stack was created.
    pub created_at: DateTime<Utc>,
    /// Ordered list of pull requests in this stack, from bottom to top.
    pub pull_requests: Vec<StackedPrEntry>,
}
