//! Data models for GitHub Codespaces.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces?apiVersion=2022-11-28)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{Author, CodespaceId, Repository, RepositoryId};

/// A GitHub Codespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Codespace {
    pub id: CodespaceId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    pub owner: Author,
    pub billable_owner: Author,
    pub repository: Repository,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<CodespaceMachine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuild: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    pub state: String,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_status: Option<CodespaceGitStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<u32>,
    pub web_url: Url,
    pub machines_url: Url,
    pub start_url: Url,
    pub stop_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulls_url: Option<Url>,
    #[serde(default)]
    pub recent_folders: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_notice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_expires_at: Option<DateTime<Utc>>,
}

/// A machine type available for a codespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespaceMachine {
    pub name: String,
    pub display_name: String,
    pub operating_system: String,
    pub storage_in_bytes: u64,
    pub memory_in_bytes: u64,
    pub cpus: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuild_availability: Option<String>,
}

/// Git status of a codespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespaceGitStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ahead: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behind: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_unpushed_changes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_uncommitted_changes: Option<bool>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
}

/// A devcontainer configuration in a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Devcontainer {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// Default attributes for a new codespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespaceDefaultAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_owner: Option<Author>,
    pub defaults: CodespaceDefaults,
}

/// Default settings inside [`CodespaceDefaultAttributes`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespaceDefaults {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<String>,
}

/// Result of a codespace devcontainer permissions check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespacePermissionsCheck {
    pub accepted: bool,
}

/// Details of a codespace export operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespaceExportDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
}

/// Access control configuration for organization codespaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgCodespacesAccess {
    pub visibility: OrgCodespacesAccessVisibility,
}

/// Visibility setting for organization codespaces access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrgCodespacesAccessVisibility {
    Disabled,
    SelectedMembers,
    AllMembers,
    AllMembersAndOutsideCollaborators,
}

/// A secret available to Codespaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodespacesSecret {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub visibility: crate::models::orgs::secrets::Visibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

/// A list of selected repositories that have access to a secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelectedRepositories {
    pub total_count: u32,
    pub repositories: Vec<Repository>,
}

/// Reference to a pull request used when creating a codespace.
#[derive(Debug, Clone, Serialize)]
pub struct CodespacePullRequestRef {
    pub pull_request_number: u64,
    pub repository_id: RepositoryId,
}

/// Request body for creating a codespace for the authenticated user.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserCodespace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<RepositoryId>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_repo_permissions_opt_out: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<CodespacePullRequestRef>,
}

/// Request body for creating a codespace in a repository.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoCodespace {
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_repo_permissions_opt_out: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_minutes: Option<u32>,
}

/// Request body for creating a codespace from a pull request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePullRequestCodespace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_repo_permissions_opt_out: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_minutes: Option<u32>,
}

/// Request body for updating a codespace.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCodespace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_folders: Option<Vec<String>>,
}

/// Request body for publishing a codespace.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PublishCodespace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

/// Request body for creating or updating a user codespaces secret.
#[derive(Debug, Clone, Serialize)]
pub struct CreateUserCodespacesSecret<'a> {
    pub encrypted_value: &'a str,
    pub key_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repository_ids: Option<&'a [RepositoryId]>,
}

/// Request body for replacing selected repositories for a secret.
#[derive(Debug, Clone, Serialize)]
pub struct SetSelectedRepositories<'a> {
    pub selected_repository_ids: &'a [RepositoryId],
}
