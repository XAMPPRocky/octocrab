use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{ImportAuthorId, MigrationId, Repository, SimpleUser};

/// A migration object representing an export of organization or user data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Migration {
    pub id: MigrationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<SimpleUser>,
    pub guid: String,
    pub state: String,
    pub lock_repositories: bool,
    #[serde(default)]
    pub exclude_metadata: bool,
    #[serde(default)]
    pub exclude_git_data: bool,
    #[serde(default)]
    pub exclude_attachments: bool,
    #[serde(default)]
    pub exclude_releases: bool,
    #[serde(default)]
    pub exclude_owner_projects: bool,
    #[serde(default)]
    pub org_metadata_only: bool,
    #[serde(default)]
    pub repositories: Vec<Repository>,
    pub url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

/// Request payload to start a user or organization migration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartMigration {
    pub repositories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_git_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_attachments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_releases: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_owner_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_metadata_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

impl StartMigration {
    pub fn new(repositories: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            repositories: repositories.into_iter().map(Into::into).collect(),
            lock_repositories: None,
            exclude_metadata: None,
            exclude_git_data: None,
            exclude_attachments: None,
            exclude_releases: None,
            exclude_owner_projects: None,
            org_metadata_only: None,
            exclude: None,
        }
    }

    pub fn lock_repositories(mut self, lock: bool) -> Self {
        self.lock_repositories = Some(lock);
        self
    }

    pub fn exclude_metadata(mut self, exclude: bool) -> Self {
        self.exclude_metadata = Some(exclude);
        self
    }

    pub fn exclude_git_data(mut self, exclude: bool) -> Self {
        self.exclude_git_data = Some(exclude);
        self
    }

    pub fn exclude_attachments(mut self, exclude: bool) -> Self {
        self.exclude_attachments = Some(exclude);
        self
    }

    pub fn exclude_releases(mut self, exclude: bool) -> Self {
        self.exclude_releases = Some(exclude);
        self
    }

    pub fn exclude_owner_projects(mut self, exclude: bool) -> Self {
        self.exclude_owner_projects = Some(exclude);
        self
    }

    pub fn org_metadata_only(mut self, metadata_only: bool) -> Self {
        self.org_metadata_only = Some(metadata_only);
        self
    }

    pub fn exclude(mut self, exclude: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude = Some(exclude.into_iter().map(Into::into).collect());
        self
    }
}

/// A repository import from an external source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Import {
    pub vcs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_lfs: Option<bool>,
    pub vcs_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svc_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svn_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
    pub status: ImportStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_step: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_large_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_files_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_files_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_choices: Option<Vec<ImportProjectChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors_count: Option<i64>,
    pub url: Url,
    pub html_url: Url,
    pub authors_url: Url,
    pub repository_url: Url,
}

/// The status of a repository import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ImportStatus {
    Auth,
    Error,
    None,
    Detecting,
    Choose,
    AuthFailed,
    Importing,
    Mapping,
    WaitingToPush,
    Pushing,
    Complete,
    Setup,
    Unknown,
    DetectionFoundMultiple,
    DetectionFoundNothing,
    DetectionNeedsAuth,
    #[serde(other)]
    Other,
}

/// Project choice during repository import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportProjectChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_name: Option<String>,
}

/// Request body to start an import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartImport {
    pub vcs_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
}

impl StartImport {
    pub fn new(vcs_url: impl Into<String>) -> Self {
        Self {
            vcs_url: vcs_url.into(),
            vcs: None,
            vcs_username: None,
            vcs_password: None,
            tfvc_project: None,
        }
    }

    pub fn vcs(mut self, vcs: impl Into<String>) -> Self {
        self.vcs = Some(vcs.into());
        self
    }

    pub fn vcs_username(mut self, username: impl Into<String>) -> Self {
        self.vcs_username = Some(username.into());
        self
    }

    pub fn vcs_password(mut self, password: impl Into<String>) -> Self {
        self.vcs_password = Some(password.into());
        self
    }

    pub fn tfvc_project(mut self, project: impl Into<String>) -> Self {
        self.tfvc_project = Some(project.into());
        self
    }
}

/// Request body to update an import.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateImport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
}

impl UpdateImport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vcs_username(mut self, username: impl Into<String>) -> Self {
        self.vcs_username = Some(username.into());
        self
    }

    pub fn vcs_password(mut self, password: impl Into<String>) -> Self {
        self.vcs_password = Some(password.into());
        self
    }

    pub fn vcs(mut self, vcs: impl Into<String>) -> Self {
        self.vcs = Some(vcs.into());
        self
    }

    pub fn tfvc_project(mut self, project: impl Into<String>) -> Self {
        self.tfvc_project = Some(project.into());
        self
    }
}

/// A commit author in a repository import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportAuthor {
    pub id: ImportAuthorId,
    pub remote_id: String,
    pub remote_name: String,
    pub email: String,
    pub name: String,
    pub url: Url,
    pub import_url: Url,
}

/// Request body to map a commit author.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MapCommitAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MapCommitAuthor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// A large file found during repository import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LargeFile {
    pub ref_name: String,
    pub path: String,
    pub oid: String,
    pub size: i64,
}

/// Preference for using Git LFS during import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsPreference {
    OptIn,
    OptOut,
}
