use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeScanningAlert {
    pub number: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub url: String,
    pub html_url: String,
    pub state: String,
    pub fixed_at: Option<String>,
    pub dismissed_by: Dismisser,
    pub dismissed_at: String,
    pub dismissed_reason: String,
    pub dismissed_comment: String,
    pub rule: Rule,
    pub tool: Tool,
    pub most_recent_instance: MostRecentInstance,
    pub instances_url: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Dismisser {
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
    pub url: Url,
    pub html_url: Url,
    pub followers_url: Url,
    pub following_url: Url,
    pub gists_url: Url,
    pub starred_url: Url,
    pub subscriptions_url: Url,
    pub organizations_url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub received_events_url: Url,
    pub r#type: String,
    pub site_admin: bool,
    pub patch_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Rule {
    pub id: String,
    pub severity: String,
    pub description: String,
    pub name: String,
    pub tags: Vec<String>,
    pub security_severity_level: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Tool {
    pub name: String,
    pub guid: Option<String>,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MostRecentInstance {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub analysis_key: String,
    pub environment: Environment,
    pub category: String,
    pub state: String,
    pub commit_sha: String,
    pub message: Message,
    pub location: Location,
    pub classifications: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Environment {
    #[serde(rename = "build-mode")]
    pub build_mode: Option<String>,
    pub category: Option<String>,
    pub language: Option<String>,
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
