use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{Author, PatId, PatRequestId};

/// A fine-grained personal access token associated with an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgPersonalAccessToken {
    pub id: PatId,
    pub owner: Author,
    #[serde(default)]
    pub repository_selection: Option<String>,
    #[serde(default)]
    pub repositories_url: Option<String>,
    #[serde(default)]
    pub token_expired: Option<bool>,
    #[serde(default)]
    pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub token_last_used_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub access_granted_at: Option<DateTime<Utc>>,
}

/// A request for a personal access token to access organization resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgPersonalAccessTokenRequest {
    pub id: PatRequestId,
    pub owner: Author,
    #[serde(default)]
    pub repository_selection: Option<String>,
    #[serde(default)]
    pub repositories_url: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_expired: Option<bool>,
    #[serde(default)]
    pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub token_last_used_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
}

/// Action to perform on a personal access token (e.g. revoke).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatAction {
    Revoke,
}

/// Decision for reviewing a personal access token request (approve or deny).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatReviewDecision {
    Approve,
    Deny,
}
