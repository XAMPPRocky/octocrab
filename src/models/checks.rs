use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRun {
    pub id: CheckRunId,
    pub node_id: String,
    pub details_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListCheckRuns {
    pub total_count: u64,
    pub check_runs: Vec<CheckRun>,
}
