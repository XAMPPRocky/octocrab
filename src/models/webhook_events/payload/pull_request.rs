use serde::{Deserialize, Serialize};

use crate::models::{pulls::PullRequest, teams::RequestedTeam, Author, Label, Milestone};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestWebhookEventPayload {
    pub action: PullRequestWebhookEventAction,
    pub assignee: Option<Author>,
    pub enterprise: Option<serde_json::Value>,
    pub number: u64,
    pub pull_request: PullRequest,
    pub reason: Option<String>,
    pub milestone: Option<Milestone>,
    pub label: Option<Label>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub changes: Option<PullRequestWebhookEventChanges>,
    pub requested_reviewer: Option<Author>,
    pub requested_team: Option<RequestedTeam>,
    /// Top-level stack object. Only present on `stacked` action events.
    ///
    /// Requires the `stack-prs` Cargo feature.
    #[cfg(feature = "stack-prs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Box<crate::models::pr_stacks::PrStack>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestWebhookEventAction {
    Assigned,
    AutoMergeDisabled,
    AutoMergeEnabled,
    Closed,
    ConvertedToDraft,
    Demilestoned,
    Dequeued,
    Edited,
    Enqueued,
    Labeled,
    Locked,
    Milestoned,
    Opened,
    ReadyForReview,
    Reopened,
    ReviewRequestRemoved,
    ReviewRequested,
    Synchronize,
    Unassigned,
    Unlabeled,
    Unlocked,
    /// A pull request was added to or removed from a stack.
    ///
    /// The `stack` field in the event payload contains the updated stack.
    /// Requires the `stack-prs` Cargo feature.
    #[cfg(feature = "stack-prs")]
    Stacked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestWebhookEventChanges {
    base: Option<PullRequestWebhookEventBase>,
    body: Option<OldValue<String>>,
    title: Option<OldValue<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestWebhookEventBase {
    #[serde(rename(deserialize = "ref"))]
    #[serde(rename(serialize = "ref"))]
    pub ref_: OldValue<String>,
    pub sha: OldValue<String>,
}
