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
    Tag { sha: String, url: Url },
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<GitUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<GitUser>,
}

/// The author of a commit, identified by its name and email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitUser {
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
    pub name: String,
    pub commit: CommitObject,
    pub zipball_url: Url,
    pub tarball_url: Url,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct CommitObject {
    pub sha: String,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    pub url: Url,
    pub html_url: Url,
    pub assets_url: Url,
    pub upload_url: Url,
    pub tarball_url: Option<Url>,
    pub zipball_url: Option<Url>,
    pub id: ReleaseId,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub author: crate::models::User,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    pub url: Url,
    pub browser_download_url: Url,
    pub id: AssetId,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uploader: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TagInfo {
    pub node_id: String,
    pub tag: String,
    pub sha: String,
    pub url: Url,
    pub message: String,
}
