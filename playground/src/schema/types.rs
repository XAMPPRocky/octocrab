use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    String {
        placeholder: Option<String>,
    },
    Integer {
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
    },
    Boolean {
        default: bool,
    },
    Enum {
        options: Vec<String>,
        default: Option<String>,
    },
    Secret {
        placeholder: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldSchema {
    pub key: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub required: bool,
    pub field_type: FieldType,
    /// When true, child nodes can automatically inherit this parameter from an ancestor
    pub inheritable: bool,
}

#[allow(dead_code)]
impl FieldSchema {
    pub fn new_string(
        key: &'static str,
        label: &'static str,
        description: &'static str,
        required: bool,
        placeholder: Option<&str>,
        inheritable: bool,
    ) -> Self {
        Self {
            key,
            label,
            description,
            required,
            field_type: FieldType::String {
                placeholder: placeholder.map(|s| s.to_string()),
            },
            inheritable,
        }
    }

    pub fn new_integer(
        key: &'static str,
        label: &'static str,
        description: &'static str,
        required: bool,
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Self {
        Self {
            key,
            label,
            description,
            required,
            field_type: FieldType::Integer { min, max, default },
            inheritable: false,
        }
    }

    pub fn new_boolean(
        key: &'static str,
        label: &'static str,
        description: &'static str,
        default: bool,
    ) -> Self {
        Self {
            key,
            label,
            description,
            required: false,
            field_type: FieldType::Boolean { default },
            inheritable: false,
        }
    }

    pub fn new_enum(
        key: &'static str,
        label: &'static str,
        description: &'static str,
        options: &[&str],
        default: Option<&str>,
    ) -> Self {
        Self {
            key,
            label,
            description,
            required: false,
            field_type: FieldType::Enum {
                options: options.iter().map(|s| s.to_string()).collect(),
                default: default.map(|s| s.to_string()),
            },
            inheritable: false,
        }
    }
}
