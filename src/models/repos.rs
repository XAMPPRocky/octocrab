use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Ref {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub node_id: String,
    pub url: Url,
    pub object: Object,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Object {
    Commit { sha: String, url: Url },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    pub author: AuthorUser,
    pub committer: AuthorUser,
}

/// The author of a commit, identified by its name and email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUpdate {
    pub content: Content,
    pub commit: Commit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Content {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i64,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub r#type: String,
    #[serde(rename = "_links")]
    pub links: ContentLinks,
    pub license: Option<License>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContentLinks {
    pub git: Url,
    pub html: Url,
    #[serde(rename = "self")]
    pub _self: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Tag {
    name: String,
    commit: CommitObject,
    zipball_url: Url,
    tarball_url: Url,
    node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct CommitObject {
    sha: String,
    url: Url,
}
