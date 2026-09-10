use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{Repository, SimpleUser};
use crate::models::teams::Team;

/// A GitHub Global Security Advisory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityAdvisory {
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    pub url: Url,
    pub html_url: Url,
    pub repository_advisory_url: Option<Url>,
    pub summary: String,
    pub description: Option<String>,
    pub r#type: String,
    pub severity: String,
    pub source_code_location: Option<Url>,
    pub identifiers: Option<Vec<SecurityAdvisoryIdentifier>>,
    pub references: Option<Vec<String>>,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub github_reviewed_at: Option<DateTime<Utc>>,
    pub nvd_published_at: Option<DateTime<Utc>>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub vulnerabilities: Option<Vec<Vulnerability>>,
    pub cvss_severities: Option<CvssSeverities>,
    pub cwes: Option<Vec<Cwe>>,
    pub credits: Option<Vec<GlobalAdvisoryCredit>>,
    pub epss: Option<Epss>,
}

/// A GitHub Repository Security Advisory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryAdvisory {
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    pub url: Url,
    pub html_url: Url,
    pub summary: String,
    pub description: Option<String>,
    pub severity: Option<String>,
    pub author: Option<SimpleUser>,
    pub publisher: Option<SimpleUser>,
    pub identifiers: Vec<SecurityAdvisoryIdentifier>,
    pub state: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub submission: Option<Submission>,
    pub vulnerabilities: Option<Vec<Vulnerability>>,
    pub cvss_severities: Option<CvssSeverities>,
    pub cwes: Option<Vec<Cwe>>,
    pub cwe_ids: Option<Vec<String>>,
    pub credits: Option<Vec<RepositoryAdvisoryCredit>>,
    pub credits_detailed: Option<Vec<RepositoryAdvisoryCreditDetailed>>,
    pub collaborating_users: Option<Vec<SimpleUser>>,
    pub collaborating_teams: Option<Vec<Team>>,
    pub private_fork: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityAdvisoryIdentifier {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Vulnerability {
    pub package: Option<VulnerabilityPackage>,
    pub vulnerable_version_range: Option<String>,
    pub first_patched_version: Option<String>,
    pub patched_versions: Option<String>,
    pub vulnerable_functions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VulnerabilityPackage {
    pub ecosystem: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CvssSeverities {
    pub cvss_v3: Option<CvssScore>,
    pub cvss_v4: Option<CvssScore>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CvssScore {
    pub vector_string: Option<String>,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cwe {
    pub cwe_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Submission {
    pub accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GlobalAdvisoryCredit {
    pub user: SimpleUser,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryAdvisoryCredit {
    pub login: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryAdvisoryCreditDetailed {
    pub user: SimpleUser,
    pub r#type: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Epss {
    pub percentage: Option<f64>,
    pub percentile: Option<f64>,
}

/// Request body for creating a repository security advisory.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateRepositoryAdvisory {
    pub summary: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_id: Option<String>,
    pub vulnerabilities: Vec<CreateVulnerability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<Vec<CreateCredit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss_vector_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_private_fork: Option<bool>,
}

/// Request body for updating a repository security advisory.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateRepositoryAdvisory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<CreateVulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<Vec<CreateCredit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss_vector_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborating_users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborating_teams: Option<Vec<String>>,
}

/// Request body for privately reporting a security vulnerability.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReportVulnerability {
    pub summary: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<CreateVulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss_vector_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_private_fork: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateVulnerability {
    pub package: CreatePackage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_version_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patched_versions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_functions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreatePackage {
    pub ecosystem: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateCredit {
    pub login: String,
    pub r#type: String,
}
