use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{apps::App, repos::RepoCommit, teams::Team, Author};

/// Detailed information about a branch returned by get and rename endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DetailedBranch {
    pub name: String,
    pub commit: RepoCommit,
    pub protected: bool,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub links: Option<BranchLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<BranchProtectionSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_url: Option<Url>,
}

/// Links associated with a branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchLinks {
    pub html: Url,
    #[serde(rename = "self")]
    pub self_link: Url,
}

/// Summary of protection settings returned inside [`DetailedBranch`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtectionSummary {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<BranchProtectionSummaryStatusChecks>,
}

/// Summary of required status checks returned inside [`BranchProtectionSummary`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtectionSummaryStatusChecks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_level: Option<String>,
    #[serde(default)]
    pub contexts: Vec<String>,
    #[serde(default)]
    pub checks: Vec<StatusCheck>,
}

/// A required status check entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusCheck {
    pub context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<u64>,
}

impl StatusCheck {
    pub fn new(context: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            app_id: None,
        }
    }

    pub fn with_app_id(context: impl Into<String>, app_id: u64) -> Self {
        Self {
            context: context.into(),
            app_id: Some(app_id),
        }
    }
}

/// Complete branch protection settings for a protected branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<RequiredStatusChecks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<AdminEnforcement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<BranchProtectionRestrictions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_creations: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_branch: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fork_syncing: Option<ProtectionFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_signatures: Option<ProtectionFlag>,
}

/// A boolean protection flag with an optional URL (e.g. required_signatures, linear_history).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProtectionFlag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    pub enabled: bool,
}

/// Admin enforcement setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AdminEnforcement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    pub enabled: bool,
}

/// Required status checks configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequiredStatusChecks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    pub strict: bool,
    #[serde(default)]
    pub contexts: Vec<String>,
    #[serde(default)]
    pub checks: Vec<StatusCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_level: Option<String>,
}

/// Required pull request review settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequiredPullRequestReviews {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_last_push_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<DismissalRestrictions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_pull_request_allowances: Option<BypassPullRequestAllowances>,
}

/// Dismissal restrictions configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DismissalRestrictions {
    #[serde(default)]
    pub users: Vec<Author>,
    #[serde(default)]
    pub teams: Vec<Team>,
    #[serde(default)]
    pub apps: Vec<App>,
}

/// Bypass pull request allowances configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BypassPullRequestAllowances {
    #[serde(default)]
    pub users: Vec<Author>,
    #[serde(default)]
    pub teams: Vec<Team>,
    #[serde(default)]
    pub apps: Vec<App>,
}

/// Access restrictions for a protected branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtectionRestrictions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_url: Option<Url>,
    #[serde(default)]
    pub users: Vec<Author>,
    #[serde(default)]
    pub teams: Vec<Team>,
    #[serde(default)]
    pub apps: Vec<App>,
}

/// Request body for setting or updating status check contexts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextsRequest {
    pub contexts: Vec<String>,
}

/// Request body for setting or updating user restrictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsersRestrictionsRequest {
    pub users: Vec<String>,
}

/// Request body for setting or updating team restrictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamsRestrictionsRequest {
    pub teams: Vec<String>,
}

/// Request body for setting or updating app restrictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppsRestrictionsRequest {
    pub apps: Vec<String>,
}

/// Request body for updating required status checks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStatusChecks {
    pub strict: bool,
    pub contexts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<StatusCheck>>,
}

/// Request body for updating pull request reviews.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatePullRequestReviews {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_last_push_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<DismissalRestrictionsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_pull_request_allowances: Option<BypassPullRequestAllowancesRequest>,
}

/// Request body for dismissal restrictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DismissalRestrictionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<String>>,
}

/// Request body for bypass pull request allowances.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BypassPullRequestAllowancesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<String>>,
}

/// Request body for updating push access restrictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateRestrictions {
    pub users: Vec<String>,
    pub teams: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<String>>,
}
