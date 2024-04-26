use crate::models;

use super::{reactions::ReactionContent, *};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    pub html_url: Url,
    pub url: Url,
    pub id: CommentId,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    pub commit_id: String,
    pub user: Author,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<CommentReactions>,
}

/// Reactions summary of a comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct CommentReactions {
    pub url: Url,
    pub total_count: u64,
    #[serde(flatten)]
    pub reactions: Option<HashMap<ReactionContent, u64>>,
}

/// Commit Comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitComparison {
    pub ahead_by: i64,
    /// Commit
    pub base_commit: Commit,
    pub behind_by: i64,
    pub commits: Vec<Commit>,
    pub diff_url: String,
    pub files: Option<Vec<repos::DiffEntry>>,
    pub html_url: String,
    /// Commit
    pub merge_base_commit: Commit,
    pub patch_url: String,
    pub permalink_url: String,
    pub status: GithubCommitStatus,
    pub total_commits: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitElement {
    pub author: Option<GitUser>,
    pub comment_count: i64,
    pub committer: Option<GitUser>,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub verification: Option<Verification>,
}

/// Metaproperties for Git author/committer information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitUser {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    pub payload: Option<String>,
    pub reason: String,
    pub signature: Option<String>,
    pub verified: bool,
}

#[deprecated(note = "use repos::DiffEntryStatus instead")]
pub type FileStatus = repos::DiffEntryStatus;

/// Commit
#[deprecated(note = "use repos::DiffEntry instead")]
pub type CommitFile = repos::DiffEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitParent {
    pub html_url: Option<String>,
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitStats {
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub total: Option<i64>,
}

/// Commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub author: Option<Author>,
    pub comments_url: String,
    pub commit: CommitElement,
    pub committer: Option<Author>,
    pub files: Option<Vec<repos::DiffEntry>>,
    pub html_url: String,
    pub node_id: String,
    pub parents: Vec<CommitParent>,
    pub sha: String,
    pub stats: Option<CommitStats>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GithubCommitStatus {
    Ahead,
    Behind,
    Diverged,
    Identical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommitObject {
    pub sha: String,
    pub node_id: String,
    pub url: String,
    pub author: models::repos::CommitAuthor,
    pub committer: repos::CommitAuthor,
    pub message: String,
    pub tree: models::repos::CommitObject,
    pub parents: Vec<models::repos::Commit>,
    pub verification: models::repos::Verification,
    pub html_url: String,
}
