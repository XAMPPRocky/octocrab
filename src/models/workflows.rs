use super::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkFlow {
    pub id: WorkflowId,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Url,
    pub html_url: Url,
    pub badge_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Run {
    pub id: RunId,
    pub workflow_id: WorkflowId,
    pub node_id: String,
    pub name: String,
    pub head_branch: String,
    pub head_sha: String,
    pub run_number: i64,
    pub event: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Url,
    pub html_url: Url,
    pub jobs_url: Url,
    pub logs_url: Url,
    pub check_suite_url: Url,
    pub artifacts_url: Url,
    pub cancel_url: Url,
    pub rerun_url: Url,
    pub workflow_url: Url,
    // This is not correct! E.g. I see stuff that is only a subset of PullRequest, e.g.
    // "pull_requests":[{"url":"https://api.github.com/repos/artichoke/artichoke/pulls/1346","id":717179206,"number":1346,"head":{"ref":"pernosco-integration","sha":"9ee4335ecfc3e7abe44bddadb117a23d0d63e4ee","repo":{"id":199196552,"url":"https://api.github.com/repos/artichoke/artichoke","name":"artichoke"}},"base":{"ref":"trunk","sha":"abbb7cf0c75ab51b84309ac547c3c3c089dd36eb","repo":{"id":199196552,"url":"https://api.github.com/repos/artichoke/artichoke","name":"artichoke"}}}]
    // pub pull_requests: Vec<super::pulls::PullRequest>,
    // TODO: other attrs
    // ref: https://docs.github.com/en/rest/reference/actions#list-workflow-runs
    pub head_commit: HeadCommit,
    pub repository: Repository,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HeadCommit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub author: super::repos::CommitAuthor,
    pub committer: super::repos::CommitAuthor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Job {
    pub id: JobId,
    pub run_id: RunId,
    pub node_id: String,
    pub head_sha: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    // Github has been seen to set null here during Job startup
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub name: String,
    pub url: Url,
    pub html_url: Url,
    pub run_url: Url,
    pub check_run_url: Url,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Step {
    pub name: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    pub number: i64,
    // Github might set null here during Step startup...
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowListArtifact {
    pub id: crate::models::ArtifactId,
    pub node_id: String,
    pub name: String,
    pub size_in_bytes: usize,
    pub url: Url,
    pub archive_download_url: Url,
    pub expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct WorkflowDispatch {
    pub r#ref: String,
    pub inputs: serde_json::Value,
}
