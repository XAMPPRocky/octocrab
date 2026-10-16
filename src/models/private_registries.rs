use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A private registry configuration for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgPrivateRegistry {
    pub name: String,
    pub registry_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_repository_ids: Option<Vec<u64>>,
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
}

/// Response returned when listing private registries for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgPrivateRegistriesResponse {
    pub total_count: u64,
    pub configurations: Vec<OrgPrivateRegistry>,
}

/// A private registry public key for an organization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgPrivateRegistryPublicKey {
    pub key_id: String,
    pub key: String,
}
