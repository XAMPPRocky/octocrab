use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{reactions::ReactionContent, Author, AuthorAssociation, CommentId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitCommentWebhookEventPayload {
    pub action: CommitCommentWebhookEventAction,
    pub comment: CommitComment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CommitCommentWebhookEventAction {
    Created,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct CommitComment {
    pub author_association: AuthorAssociation,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub html_url: Url,
    pub id: CommentId,
    pub line: Option<u64>,
    pub node_id: String,
    pub path: Option<String>,
    pub position: Option<u64>,
    pub reactions: Option<HashMap<ReactionContent, u64>>,
    pub updated_at: String,
    pub url: Url,
    pub user: Option<Author>,
}
