use serde::{Deserialize, Serialize};

pub use crate::models::repos::custom_properties::CustomPropertyValue;
use crate::models::RepositoryId;

/// A custom property definition for an organization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgCustomPropertyDefinition {
    pub property_name: String,
    pub value_type: CustomPropertyValueType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values_editable_by: Option<ValuesEditableBy>,
}

/// The value type of a custom property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomPropertyValueType {
    String,
    SingleSelect,
    MultiSelect,
    TrueFalse,
}

/// Who can edit values of a custom property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValuesEditableBy {
    OrgActors,
    OrgAndRepoActors,
}

/// Custom property values for an organization repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgRepoCustomPropertyValues {
    pub repository_id: RepositoryId,
    pub repository_name: String,
    pub repository_full_name: String,
    pub properties: Vec<CustomPropertyValue>,
}
