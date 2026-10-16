use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Author;

/// A Project (v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectV2 {
    pub id: u64,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Author>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_status_update: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
}

/// An option on a single select field in a Project (v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectV2FieldOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A field in a Project (v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectV2Field {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ProjectV2FieldOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// An item in a Project (v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectV2Item {
    pub id: u64,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

/// A view in a Project (v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectV2View {
    pub id: u64,
    pub number: u64,
    pub name: String,
    pub layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_fields: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_group_by: Option<Vec<u64>>,
}

/// Response wrapper for operations returning `{ "value": T }`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectV2ValueResponse<T> {
    pub value: T,
}

/// Helper enum to deserialize either `{ "value": ProjectV2Item }` or `ProjectV2Item`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectV2ItemResponse {
    Wrapped { value: ProjectV2Item },
    Direct(ProjectV2Item),
}

impl From<ProjectV2ItemResponse> for ProjectV2Item {
    fn from(res: ProjectV2ItemResponse) -> Self {
        match res {
            ProjectV2ItemResponse::Wrapped { value } => value,
            ProjectV2ItemResponse::Direct(item) => item,
        }
    }
}

/// Helper enum to deserialize either `{ "value": ProjectV2View }` or `ProjectV2View`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectV2ViewResponse {
    Wrapped { value: ProjectV2View },
    Direct(ProjectV2View),
}

impl From<ProjectV2ViewResponse> for ProjectV2View {
    fn from(res: ProjectV2ViewResponse) -> Self {
        match res {
            ProjectV2ViewResponse::Wrapped { value } => value,
            ProjectV2ViewResponse::Direct(view) => view,
        }
    }
}
