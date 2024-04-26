use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeScanningAlertWebhookEventPayload {
    pub action: CodeScanningAlertWebhookEventAction,
    pub alert: serde_json::Value,
    /// The commit SHA of the code scanning alert. When the action is reopened_by_user or closed_by_user, the event was triggered by the sender and this value will be empty.
    pub commit_oid: String,
    pub enterprise: Option<serde_json::Value>,
    /// The Git reference of the code scanning alert. When the action is reopened_by_user or closed_by_user, the event was triggered by the sender and this value will be empty.
    pub r#ref: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CodeScanningAlertWebhookEventAction {
    AppearedInBranch,
    ClosedByUser,
    Created,
    Fixed,
    Reopened,
    ReopenedByUser,
}
