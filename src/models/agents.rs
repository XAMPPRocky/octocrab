use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{Author, Repository};

/// An Agent automation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentAutomation {
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_actor_write_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for listing repository agent automations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentAutomationsResponse {
    pub automations: Vec<AgentAutomation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

/// Available tool category for agent automations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentAvailableTool {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
}

/// Available trigger for agent automations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentAvailableTrigger {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<serde_json::Value>>,
}

/// An agent task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentTask {
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for listing agent tasks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentTasksResponse {
    pub tasks: Vec<AgentTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

/// An Agent secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentSecret {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

/// Response for listing agent secrets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentSecretsResponse {
    pub total_count: u64,
    pub secrets: Vec<AgentSecret>,
}

/// Public key used for encrypting agent secrets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentPublicKey {
    pub key_id: String,
    pub key: String,
}

/// An Agent variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentVariable {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

/// Response for listing agent variables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentVariablesResponse {
    pub total_count: u64,
    pub variables: Vec<AgentVariable>,
}

/// Selected repositories for an organization agent secret or variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentRepositoriesResponse {
    pub total_count: u64,
    pub repositories: Vec<Repository>,
}
