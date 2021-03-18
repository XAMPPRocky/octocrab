use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkFlow {
    pub id: i64,
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
    pub id: i64,
    pub workflow_id: i64,
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
    pub pull_requests: Vec<super::pulls::PullRequest>,
    // TODO: other attrs
    // ref: https://docs.github.com/en/rest/reference/actions#list-workflow-runs
    pub head_commit: HeadCommit,
    pub repository: Repository,
    pub head_repository: Repository,
}

pub struct HeadCommit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub author: super::repos::GitUser,
    pub committer: super::repos::GitUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Job {
    pub id: i64,
    pub run_id: i64,
    pub node_id: String,
    pub head_sha: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    pub started_at: chrono::DateTime<chrono::Utc>,
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
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}
