use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A code security configuration for an enterprise or organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeSecurityConfiguration {
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced_security: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_graph: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependabot_alerts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependabot_security_updates: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_scanning_default_setup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_push_protection: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_vulnerability_reporting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::date_serde::deserialize_opt"
    )]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::date_serde::deserialize_opt"
    )]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Default code security configuration setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeSecurityDefaultConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_for_new_repos: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}

/// Repositories associated with a code security configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeSecurityConfigurationRepositories {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
}

/// Code security configuration status associated with a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoCodeSecurityConfiguration {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CodeSecurityConfiguration>,
}
