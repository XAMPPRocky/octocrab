use serde::{Deserialize, Serialize};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryWebhookEventPayload {
    pub action: RepositoryWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub changes: Option<RepositoryWebhookEventChanges>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RepositoryWebhookEventAction {
    Archived,
    Created,
    Deleted,
    Edited,
    Privatized,
    Publicized,
    Renamed,
    Transferred,
    Unarchived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryWebhookEventChanges {
    pub default_branch: Option<OldValue<String>>,
    pub description: Option<OldValue<Option<String>>>,
    pub homepage: Option<OldValue<Option<String>>>,
    pub topics: Option<OldValue<Option<Vec<String>>>>,
}
