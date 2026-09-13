//! Data models for GitHub Dependabot.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot?apiVersion=2022-11-28)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use super::security_advisories::{CvssScore, CvssSeverities, Cwe, SecurityAdvisoryIdentifier};
use super::{Repository, RepositoryId, SimpleUser};

/// A GitHub Dependabot Alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotAlert {
    pub number: u64,
    pub state: DependabotAlertState,
    pub dependency: DependabotAlertDependency,
    pub security_advisory: DependabotAlertSecurityAdvisory,
    pub security_vulnerability: Option<DependabotAlertVulnerability>,
    pub url: Url,
    pub html_url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub dismissed_by: Option<SimpleUser>,
    pub dismissed_reason: Option<DependabotAlertDismissedReason>,
    pub dismissed_comment: Option<String>,
    pub fixed_at: Option<DateTime<Utc>>,
    pub auto_dismissed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub assignees: Vec<SimpleUser>,
    pub repository: Option<Repository>,
}

/// The state of a Dependabot alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependabotAlertState {
    AutoDismissed,
    Dismissed,
    Fixed,
    Open,
}

/// The state to set when updating a Dependabot alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependabotAlertUpdateState {
    Dismissed,
    Open,
}

/// The scope of a vulnerable dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependabotAlertScope {
    Development,
    Runtime,
}

/// The relationship of the vulnerable dependency to the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependabotAlertRelationship {
    Direct,
    Transitive,
    Unknown,
}

/// The reason for dismissing a Dependabot alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependabotAlertDismissedReason {
    FixStarted,
    Inaccurate,
    NoBandwidth,
    NotUsed,
    TolerableRisk,
}

/// Details of a dependency associated with a Dependabot alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotAlertDependency {
    pub package: DependabotAlertPackage,
    pub manifest_path: String,
    pub scope: Option<DependabotAlertScope>,
    pub relationship: Option<DependabotAlertRelationship>,
}

/// Details of a package associated with a Dependabot alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotAlertPackage {
    #[serde(default)]
    pub ecosystem: String,
    pub name: String,
}

/// Details of the security advisory associated with a Dependabot alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotAlertSecurityAdvisory {
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    pub summary: String,
    pub description: String,
    #[serde(default)]
    pub vulnerabilities: Vec<DependabotAlertVulnerability>,
    pub severity: String,
    pub cvss: Option<CvssScore>,
    pub cvss_severities: Option<CvssSeverities>,
    pub epss: Option<DependabotEpss>,
    #[serde(default)]
    pub cwes: Vec<Cwe>,
    #[serde(default)]
    pub identifiers: Vec<SecurityAdvisoryIdentifier>,
    #[serde(default)]
    pub references: Vec<DependabotReference>,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
}

/// A vulnerability entry within a Dependabot security advisory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotAlertVulnerability {
    pub package: DependabotAlertPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<DependabotFirstPatchedVersion>,
}

/// First patched version identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotFirstPatchedVersion {
    pub identifier: String,
}

/// EPSS information in a Dependabot security advisory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotEpss {
    pub percentage: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_percentile")]
    pub percentile: Option<f64>,
}

fn deserialize_percentile<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumOrStr {
        Num(f64),
        Str(String),
    }

    match Option::<NumOrStr>::deserialize(deserializer)? {
        Some(NumOrStr::Num(n)) => Ok(Some(n)),
        Some(NumOrStr::Str(s)) => s.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// A reference link in a Dependabot security advisory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotReference {
    pub url: Url,
}

/// Selected repositories for an organization Dependabot secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelectedRepositories {
    pub total_count: i32,
    pub repositories: Vec<Repository>,
}

/// Request body for setting selected repositories for an organization Dependabot secret.
#[derive(Debug, Clone, Serialize)]
pub struct SetSelectedRepositories<'a> {
    pub selected_repository_ids: &'a [RepositoryId],
}

/// Default repository access level for Dependabot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependabotDefaultRepositoryAccessLevel {
    Public,
    Internal,
}

/// Accessible repositories and default access level for Dependabot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotRepositoryAccess {
    pub default_level: DependabotDefaultRepositoryAccessLevel,
    #[serde(default)]
    pub accessible_repositories: Vec<Repository>,
}

/// Request body for updating Dependabot's repository access list.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDependabotRepositoryAccess<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_ids_to_add: Option<&'a [RepositoryId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_ids_to_remove: Option<&'a [RepositoryId]>,
}

/// Request body for setting the default repository access level for Dependabot.
#[derive(Debug, Clone, Serialize)]
pub struct SetDependabotDefaultRepositoryAccessLevel {
    pub default_level: DependabotDefaultRepositoryAccessLevel,
}
