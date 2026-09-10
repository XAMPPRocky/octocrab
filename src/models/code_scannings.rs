use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeScanningAlert {
    /// The unique identifier of the code scanning alert.
    pub number: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub url: Url,
    pub html_url: Url,
    pub state: CodeScanningState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_by: Option<SimpleUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_reason: Option<DismissedReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_comment: Option<String>,
    pub rule: Rule,
    pub tool: Tool,
    pub most_recent_instance: MostRecentInstance,
    pub instances_url: Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_approved_by: Option<SimpleUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum CodeScanningState {
    Open,
    Dismissed,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum DismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "won't fix")]
    WonTFix,
    #[serde(rename = "used in tests")]
    UsedInTests,
    #[serde(rename = "mitigated")]
    Mitigated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    None,
    Note,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SecuritySeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Rule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_severity_level: Option<SecuritySeverityLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Tool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Classifications {
    Source,
    Generated,
    Test,
    Library,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MostRecentInstance {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub analysis_key: String,
    pub environment: String,
    pub category: String,
    pub state: Option<CodeScanningState>,
    pub commit_sha: String,
    pub message: Message,
    pub location: Location,
    pub classifications: Vec<Classifications>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Environment {
    #[serde(rename = "build-mode", skip_serializing_if = "Option::is_none")]
    pub build_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Message {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Location {
    pub path: String,
    pub start_line: i64,
    pub end_line: i64,
    pub start_column: i64,
    pub end_column: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AlertInstance {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub analysis_key: String,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CodeScanningState>,
    pub commit_sha: String,
    pub message: Message,
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<Classifications>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Analysis {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub commit_sha: String,
    pub analysis_key: String,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub error: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub results_count: u64,
    pub rules_count: u64,
    pub id: u64,
    pub url: Url,
    pub sarif_id: String,
    pub tool: Tool,
    pub deletable: bool,
    pub warning: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeleteAnalysisResponse {
    pub next_analysis_url: Option<Url>,
    pub confirm_delete_url: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeqlDatabase {
    pub id: u64,
    pub name: String,
    pub language: String,
    pub uploader: SimpleUser,
    pub content_type: String,
    pub size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_suite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDefaultSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_suite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateDefaultSetupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadSarif {
    pub commit_sha: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sarif: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SarifReceipt {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SarifAnalysis {
    pub processing_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyses_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}
