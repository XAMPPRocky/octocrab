use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InteractionLimit {
    pub limit: InteractionLimitType,
    pub origin: String,
    pub expires_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InteractionLimitType {
    ExistingUsers,
    ContributorsOnly,
    CollaboratorsOnly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InteractionLimitExpiry {
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
    SixMonths,
}

/// Pull request creation cap configuration for an organization or repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestCreationCap {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_open_pull_requests: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_drafts: Option<bool>,
}

impl PullRequestCreationCap {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            max_open_pull_requests: None,
            include_drafts: None,
        }
    }
}
