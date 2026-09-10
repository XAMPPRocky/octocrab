use super::super::*;
use chrono::{DateTime, Utc};
use url::Url;

/// The configuration for GitHub Pages for a repository.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-a-github-pages-site)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesSite {
    pub url: Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_domain_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_domain_unverified_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub custom_404: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_type: Option<PagesBuildType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesSource>,
    #[serde(default)]
    pub public: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_certificate: Option<PagesHttpsCertificate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_enforced: Option<bool>,
}

fn default_pages_path() -> String {
    "/".to_string()
}

/// The source branch and directory used to publish a Pages site.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesSource {
    pub branch: String,
    #[serde(default = "default_pages_path")]
    pub path: String,
}

impl PagesSource {
    pub fn new(branch: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            branch: branch.into(),
            path: path.into(),
        }
    }
}

/// The process in which a GitHub Pages site is built.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PagesBuildType {
    Legacy,
    Workflow,
}

/// Information about a GitHub Pages HTTPS certificate.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesHttpsCertificate {
    pub state: String,
    pub description: String,
    pub domains: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Information about a GitHub Pages build error.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageBuildError {
    #[serde(default)]
    pub message: Option<String>,
}

/// Information about a GitHub Pages build.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-github-pages-build)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageBuild {
    pub url: Url,
    pub status: String,
    pub error: PageBuildError,
    #[serde(default)]
    pub pusher: Option<Author>,
    pub commit: String,
    pub duration: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The status returned when requesting a GitHub Pages build.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#request-a-apiname-pages-build)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageBuildStatus {
    pub url: Url,
    pub status: String,
}

/// A DNS health check for a GitHub Pages site.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-a-dns-health-check-for-github-pages)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<PagesDomainHealth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_domain: Option<PagesDomainHealth>,
}

/// Detailed domain DNS health check metrics.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-a-dns-health-check-for-github-pages)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesDomainHealth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_resolves: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_proxied: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cloudflare_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_fastly_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_old_ip_address: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_a_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_cname_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_mx_records_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_apex_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_be_a_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_github_user_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_fastly: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pointed_to_github_pages_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_non_github_pages_ip_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pages_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_served_by_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responds_to_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforces_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_https_eligible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caa_error: Option<String>,
}

/// An identifier for a GitHub Pages deployment (either an integer or a commit SHA string).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PagesDeploymentId {
    Integer(u64),
    String(String),
}

impl std::fmt::Display for PagesDeploymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(id) => write!(f, "{id}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

impl From<u64> for PagesDeploymentId {
    fn from(id: u64) -> Self {
        Self::Integer(id)
    }
}

impl From<String> for PagesDeploymentId {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for PagesDeploymentId {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

/// The response when creating a GitHub Pages deployment.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#create-a-github-pages-deployment)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesDeployment {
    pub id: PagesDeploymentId,
    pub status_url: Url,
    pub page_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

/// The state of a GitHub Pages deployment status.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-the-status-of-a-github-pages-deployment)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PagesDeploymentStatusState {
    DeploymentInProgress,
    SyncingFiles,
    FinishedFileSync,
    UpdatingPages,
    PurgingCdn,
    DeploymentCancelled,
    DeploymentFailed,
    DeploymentContentFailed,
    DeploymentAttemptError,
    DeploymentLost,
    Succeed,
}

/// The status of a GitHub Pages deployment.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-the-status-of-a-github-pages-deployment)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagesDeploymentStatus {
    pub status: Option<PagesDeploymentStatusState>,
}

/// The source configuration when updating a Pages site.
///
/// Can be either a string (such as `"gh-pages"`, `"master"`, or `"master /docs"`)
/// or a structured [`PagesSource`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePagesSource {
    Legacy(String),
    Source(PagesSource),
}

impl From<PagesSource> for UpdatePagesSource {
    fn from(s: PagesSource) -> Self {
        Self::Source(s)
    }
}

impl From<String> for UpdatePagesSource {
    fn from(s: String) -> Self {
        Self::Legacy(s)
    }
}

impl From<&str> for UpdatePagesSource {
    fn from(s: &str) -> Self {
        Self::Legacy(s.to_string())
    }
}
