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
    total_count: u64,
    check_runs: Vec<CheckRun>,
}
