use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Author;

/// Copilot Cloud Agent configuration for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotCloudAgentConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_configuration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_tools: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_actions_workflow_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_firewall_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_firewall_recommended_allowlist_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_allowlist: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automations_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_write_access_for_automation_triggers: Option<bool>,
}

/// Copilot Coding Agent permissions for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotCodingAgentPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_repositories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

/// Copilot Metrics report with download links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotMetricsReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_links: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_start_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_end_day: Option<String>,
}

/// A Copilot Space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSpace {
    pub id: u64,
    pub number: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for listing Copilot Spaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSpacesResponse {
    pub spaces: Vec<CopilotSpace>,
}

/// A collaborator in a Copilot Space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSpaceCollaborator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

/// Response for listing Copilot Space collaborators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSpaceCollaboratorsResponse {
    pub collaborators: Vec<CopilotSpaceCollaborator>,
}

/// A resource in a Copilot Space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSpaceResource {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copilot_chat_attachment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for listing Copilot Space resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSpaceResourcesResponse {
    pub resources: Vec<CopilotSpaceResource>,
}
