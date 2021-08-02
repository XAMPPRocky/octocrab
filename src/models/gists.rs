use super::*;
use std::collections::BTreeMap;

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Gist {
    pub comments: u64,
    pub comments_url: Url,
    pub commits_url: Url,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub files: BTreeMap<String, GistFile>,
    pub forks_url: Url,
    pub git_pull_url: Url,
    pub git_push_url: Url,
    pub html_url: Url,
    pub id: String,
    pub node_id: String,
    pub updated_at: DateTime<Utc>,
    pub url: Url,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct GistFile {
    pub content: String,
    pub filename: String,
    pub language: String,
    pub r#type: String,
    pub raw_url: Url,
    pub size: u64,
    pub truncated: bool,
}
