use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Repository;

/// Hosted compute network configuration for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgNetworkConfiguration {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,
}

/// Response for listing hosted compute network configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgNetworkConfigurationsResponse {
    pub total_count: u64,
    pub network_configurations: Vec<OrgNetworkConfiguration>,
}

/// Hosted compute network settings resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgNetworkSettings {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Immutable releases settings for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgImmutableReleasesSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforced_repositories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

/// Selected repositories for immutable releases enforcement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgImmutableReleasesRepositoriesResponse {
    pub total_count: u64,
    pub repositories: Vec<Repository>,
}

/// Issue field for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgIssueField {
    pub id: serde_json::Value,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Issue type for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgIssueType {
    pub id: serde_json::Value,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// Artifact deployment records response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgArtifactDeploymentRecordsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_records: Option<Vec<serde_json::Value>>,
}
