mod commit_comment;
mod create;
mod delete;
mod fork;
mod gollum;
mod issue_comment;
mod issues;
mod member;
mod pull_request;
mod pull_request_review_comment;
mod push;

use crate::models::repos::GitUser;
pub use commit_comment::*;
pub use create::*;
pub use delete::*;
pub use fork::*;
pub use gollum::*;
pub use issue_comment::*;
pub use issues::*;
pub use member::*;
pub use pull_request::*;
pub use pull_request_review_comment::*;
pub use push::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// The payload in an event type.
///
/// Different event types have different payloads. Any event type not specifically part
/// of this enum will be captured in the variant `UnknownEvent` with a value of
/// [`serde_json::Value`](serde_json::Value).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventPayload {
    PushEvent(PushEventPayload),
    CreateEvent(CreateEventPayload),
    DeleteEvent(DeleteEventPayload),
    IssuesEvent(IssuesEventPayload),
    IssueCommentEvent(IssueCommentEventPayload),
    CommitCommentEvent(CommitCommentEventPayload),
    ForkEvent(ForkEventPayload),
    GollumEvent(GollumEventPayload),
    MemberEvent(MemberEventPayload),
    PullRequestEvent(Box<PullRequestEventPayload>),
    PullRequestReviewCommentEvent(Box<PullRequestReviewCommentEventPayload>),
    UnknownEvent(serde_json::Value),
}

/// A git commit in specific payload types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    pub sha: String,
    pub author: GitUser,
    pub message: String,
    pub distinct: bool,
    pub url: Url,
}
