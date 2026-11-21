use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::models::{Author, OrgRoleId};

/// Response returned when listing organization roles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgRolesResponse {
    pub total_count: usize,
    pub roles: Vec<OrgRole>,
}

/// A custom or pre-defined organization role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgRole {
    pub id: OrgRoleId,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub organization: Option<Author>,
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
}

/// An organization fine-grained permission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgFineGrainedPermission {
    pub name: String,
    pub description: String,
}
