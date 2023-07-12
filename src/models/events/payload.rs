mod commit_comment;
mod create;
mod delete;
mod fork;
mod gollum;
mod issue_comment;
mod issues;
mod member;
mod pull_request;
mod pull_request_review;
mod pull_request_review_comment;
mod push;
mod workflow_run;

use crate::models::{repos::CommitAuthor, InstallationId};
pub use commit_comment::*;
pub use create::*;
pub use delete::*;
pub use fork::*;
pub use gollum::*;
pub use issue_comment::*;
pub use issues::*;
pub use member::*;
pub use pull_request::*;
pub use pull_request_review::*;
pub use pull_request_review_comment::*;
pub use push::*;
pub use workflow_run::*;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{orgs::Organization, Author, Repository};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventInstallationPayload {
    pub id: InstallationId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrappedEventPayload {
    pub installation: Option<EventInstallationPayload>,
    pub organization: Option<Organization>,
    pub repository: Option<Repository>,
    pub sender: Option<Author>,
    #[serde(flatten)]
    pub specific: Option<EventPayload>,
}

/// The payload in an event type.
///
/// Different event types have different payloads. Any event type not specifically part
/// of this enum will be captured in the variant `UnknownEvent` with a value of
/// [`serde_json::Value`](serde_json::Value).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventPayload {
    PushEvent(Box<PushEventPayload>),
    CreateEvent(Box<CreateEventPayload>),
    DeleteEvent(Box<DeleteEventPayload>),
    IssuesEvent(Box<IssuesEventPayload>),
    IssueCommentEvent(Box<IssueCommentEventPayload>),
    CommitCommentEvent(Box<CommitCommentEventPayload>),
    ForkEvent(Box<ForkEventPayload>),
    GollumEvent(Box<GollumEventPayload>),
    MemberEvent(Box<MemberEventPayload>),
    PullRequestEvent(Box<PullRequestEventPayload>),
    PullRequestReviewEvent(Box<PullRequestReviewEventPayload>),
    PullRequestReviewCommentEvent(Box<PullRequestReviewCommentEventPayload>),
    WorkflowRunEvent(Box<WorkflowRunEventPayload>),
    UnknownEvent(Box<serde_json::Value>),
}

/// A git commit in specific payload types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    pub sha: String,
    pub author: CommitAuthor,
    pub message: String,
    pub distinct: bool,
    pub url: Url,
}
