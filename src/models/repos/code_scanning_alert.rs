use super::super::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlert {
    pub number: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub url: Url,
    pub html_url: Url,
    pub instances_url: Url,
    pub state: Option<State>,
    pub fixed_at: DateTime<Utc>,
    pub dismissed_by: Option<SimpleUser>,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub dismissed_reason: Option<DismissedReason>,
    pub dismissed_comment: Option<String>,
    pub rule: Rule,
    pub tool: Tool,
    pub most_recent_instance: MostRecentInstance,
    pub instances_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Dismissed,
    Open,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "won't fix")]
    WontFix,
    #[serde(rename = "used in tests")]
    UsedInTests,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub id: Option<String>,
    pub severity: Option<Severity>,
    pub security_severity_level: Option<SecuritySeverityLevel>
    pub tags: Vec<String>,
    pub description: String,
    pub full_description: Option<String>,
    pub name: String,
    pub help: Option<String>,
    pub help_uri: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    None,
    Note,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecuritySeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub guid: Option<String>,
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MostRecentInstance {
    pub r#ref: String,
    pub analysis_key: String,
    pub category: String,
    pub environment: String,
    pub state: Option<State>,
    pub commit_sha: String,
    pub message: Message,
    pub location: Location,
    pub html_url: Option<Url>,
    pub classifications: Option<Vec<Classifications>>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub path: String,
    pub start_line: i64,
    pub end_line: i64,
    pub start_column: i64,
    pub end_column: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Classifications {
    Source,
    Generated,
    Test,
    Library,
}