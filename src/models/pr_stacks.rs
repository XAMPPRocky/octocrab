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

// ── Stack-aware PR field ────────────────────────────────────────────────────

/// Stack membership summary embedded in a [`crate::models::pulls::PullRequest`]
/// response when the PR belongs to a stack.
///
/// Only available when the `stack-prs` Cargo feature is enabled.
///
/// See <https://docs.github.com/en/pull-requests/reference/stacked-pull-requests-rest-and-graphql-apis>
#[cfg(feature = "stack-prs")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestStackSummary {
    /// Stack number within the repository.
    pub number: u64,
    /// Zero-based position of this PR in the stack (0 = bottom).
    pub position: u64,
    /// Total number of PRs in the stack.
    pub size: u64,
    /// The ultimate base branch the entire stack targets.
    pub base: StackBase,
}

// ── Async merge models ──────────────────────────────────────────────────────

/// Merge method for an asynchronous merge request.
#[cfg(feature = "stack-prs")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AsyncMergeMethod {
    Default,
    Merge,
    Squash,
    Rebase,
}

/// Merge action strategy for an asynchronous merge request.
#[cfg(feature = "stack-prs")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AsyncMergeAction {
    Default,
    DirectMerge,
    MergeQueue,
}

/// Status of an asynchronous merge operation.
#[cfg(feature = "stack-prs")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AsyncMergeStatus {
    Pending,
    Merged,
    Enqueued,
    Failed,
}

/// Details payload of an async merge result. GitHub returns one of three
/// shapes depending on the current status.
#[cfg(feature = "stack-prs")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AsyncMergeDetails {
    /// Merge is pending or enqueued: includes the operation UUID and parameters.
    Pending {
        message: String,
        uuid: String,
        merge_method: AsyncMergeMethod,
        merge_action: AsyncMergeAction,
        expected_head_sha: String,
    },
    /// Merge succeeded: includes the resulting merge commit SHA.
    Merged { message: String, sha: String },
    /// Merge failed or was rejected.
    Failed { message: String },
}

/// Result returned by the asynchronous merge endpoints.
///
/// Returned by both:
/// - `PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge-async` (submit)
/// - `GET /repos/{owner}/{repo}/pulls/{pull_number}/merge-async/{uuid}` (poll)
#[cfg(feature = "stack-prs")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AsyncMergeResult {
    /// Current status of the merge operation.
    pub status: AsyncMergeStatus,
    /// Operation-specific details.
    pub details: AsyncMergeDetails,
}
