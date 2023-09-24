use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRun {
    pub id: CheckRunId,
    pub node_id: String,
    pub details_url: Option<String>,
    pub head_sha: String,
    pub url: String,
    pub html_url: Option<String>,
    pub conclusion: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListCheckRuns {
    pub total_count: u64,
    pub check_runs: Vec<CheckRun>,
}
