use crate::models::repos::GitUser;
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
    UnknownEvent(serde_json::Value),
}

/// The payload in a [`EventPayload::PushEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PushEventPayload {
    pub push_id: u64,
    pub size: u64,
    pub distinct_size: u64,
    pub r#ref: String,
    pub head: String,
    pub before: String,
    pub commits: Vec<Commit>,
}

/// The payload in a [`EventPayload::CreateEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateEventPayload {
    // a null ref will occur on the initial create event
    pub r#ref: Option<String>,
    pub ref_type: String,
    pub master_branch: String,
    pub description: Option<String>,
    pub pusher_type: String,
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
