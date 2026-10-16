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

// Cache retention and storage limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsCacheRetentionLimit {
    pub max_cache_retention_days: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsCacheStorageLimit {
    pub max_cache_size_gb: u32,
}

// Concurrency groups
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConcurrencyGroup {
    pub group_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_acquired_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConcurrencyGroupsResponse {
    pub total_count: u64,
    pub concurrency_groups: Vec<ConcurrencyGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConcurrencyGroupDetails {
    pub group_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_url: Option<String>,
    pub total_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_members: Option<Vec<serde_json::Value>>,
}

// Runner version deprecation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RunnerVersionDeprecation {
    pub runner_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_deprecates_at: Option<DateTime<Utc>>,
}

// Permissions extensions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ArtifactAndLogRetention {
    pub days: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_allowed_days: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForkPrContributorApproval {
    pub approval_policy: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForkPrWorkflowsPrivateRepos {
    pub run_workflows_from_fork_pull_requests: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_write_tokens_to_workflows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_secrets_and_variables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval_for_fork_pr_workflows: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgSelfHostedRunnersPermissions {
    pub enabled_repositories: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

// Policies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsPolicy {
    pub id: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionsPoliciesResponse {
    pub total_count: u64,
    pub policies: Vec<ActionsPolicy>,
}

// Hosted runners
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HostedRunner {
    pub id: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_group_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_size_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_runners: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ips: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HostedRunnersResponse {
    pub total_count: u64,
    pub runners: Vec<HostedRunner>,
}

// Runner groups
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RunnerGroup {
    pub id: RunnerGroupId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runners_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_runners_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_to_workflows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_workflows: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_restrictions_read_only: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RunnerGroupsResponse {
    pub total_count: u64,
    pub runner_groups: Vec<RunnerGroup>,
}
