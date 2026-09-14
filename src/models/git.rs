use serde::{Deserialize, Serialize};
use url::Url;

pub use crate::models::commits::GitCommitObject;
pub use crate::models::repos::{CommitAuthor, GitTag, Object, Ref, Verification};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TagObject {
    pub sha: String,
    pub r#type: String,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitBlob {
    pub content: String,
    pub encoding: String,
    pub url: Url,
    pub sha: String,
    pub size: usize,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatedBlob {
    pub url: Url,
    pub sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitTree {
    pub sha: String,
    pub url: Url,
    pub tree: Vec<GitTreeEntry>,
    pub truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitTreeEntry {
    pub path: String,
    pub mode: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    pub sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTreeEntry {
    pub path: String,
    pub mode: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CreateTreeEntry {
    pub fn new(
        path: impl Into<String>,
        mode: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            mode: mode.into(),
            r#type: r#type.into(),
            sha: None,
            content: None,
        }
    }

    pub fn with_sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
}
