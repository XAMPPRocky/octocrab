use serde::{Deserialize, Serialize};

/// A custom property value for a repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomPropertyValue {
    pub property_name: String,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
}

impl CustomPropertyValue {
    pub fn new(property_name: impl Into<String>, value: Option<serde_json::Value>) -> Self {
        Self {
            property_name: property_name.into(),
            value,
        }
    }
}
