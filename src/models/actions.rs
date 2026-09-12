use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunner {
    pub id: RunnerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_group_id: Option<RunnerGroupId>,
    pub name: String,
    pub os: String,
    pub status: String,
    pub busy: bool,
    pub labels: Vec<SelfHostedRunnerLabel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerLabel {
    pub id: RunnerLabelId,
    pub name: String,
    #[serde(rename = "type")]
    pub label_type: SelfHostedRunnerLabelType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum SelfHostedRunnerLabelType {
    ReadOnly,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerJitConfig {
    pub runner: SelfHostedRunner,
    pub encoded_jit_config: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

// Runner application downloads & labels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RunnerApplicationDownload {
    pub os: String,
    pub architecture: String,
    pub download_url: String,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerLabelsList {
    pub total_count: i64,
    pub labels: Vec<SelfHostedRunnerLabel>,
}

// Cache
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgActionsCacheUsage {
    pub total_active_caches_count: i64,
    pub total_active_caches_size_in_bytes: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoActionsCacheUsage {
    pub full_name: String,
    pub active_caches_size_in_bytes: i64,
    pub active_caches_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgActionsCacheUsageByRepository {
    pub total_count: i64,
    pub repository_cache_usages: Vec<RepoActionsCacheUsage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsCacheList {
    pub total_count: i64,
    pub actions_caches: Vec<ActionsCacheItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsCacheItem {
    pub id: i64,
    #[serde(default)]
    pub r#ref: Option<String>,
    pub key: String,
    pub version: String,
    pub last_accessed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub size_in_bytes: i64,
}

// Permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EnabledRepositories {
    All,
    None,
    Selected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AllowedActions {
    All,
    LocalOnly,
    Selected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgActionsPermissions {
    pub enabled_repositories: EnabledRepositories,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_pinning_required: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoActionsPermissions {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_actions_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_pinning_required: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsSelectedRepositories {
    pub total_count: i64,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelectedActions {
    pub github_owned_allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns_allowed: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultWorkflowPermissions {
    pub default_workflow_permissions: String,
    pub can_approve_pull_request_reviews: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPermissions {
    pub access_level: String,
}

// Variables & Environment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgVariable {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub visibility: crate::models::orgs::secrets::Visibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgVariables {
    pub total_count: i32,
    pub variables: Vec<OrgVariable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentVariables {
    pub total_count: i32,
    pub variables: Vec<EnvironmentVariable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentSecret {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentSecrets {
    pub total_count: i32,
    pub secrets: Vec<EnvironmentSecret>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateEnvironmentSecret<'a> {
    pub encrypted_value: &'a str,
    pub key_id: &'a str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateEnvironmentSecretResponse {
    Created,
    Updated,
}

// OIDC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OidcCustomSub {
    pub use_default: bool,
    #[serde(default)]
    pub include_claim_keys: Option<Vec<String>>,
}
