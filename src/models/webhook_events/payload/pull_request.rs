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
