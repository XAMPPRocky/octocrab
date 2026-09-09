use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::date_serde;

/// Information about a verified email associated with a GPG key.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VerifiedEmailInfo {
    pub email: String,
    pub verified: bool,
}

/// Information about a GPG subkey.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubKeyInfo {
    pub id: u64,
    pub primary_key_id: u64,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<VerifiedEmailInfo>,
    pub subkeys: Option<Vec<SubKeyInfo>>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_key: Option<String>,
    pub revoked: bool,
}

/// A GPG key.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/users/gpg-keys?apiVersion=2022-11-28)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GpgKey {
    pub id: u64,
    pub name: String,
    pub primary_key_id: u64,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<VerifiedEmailInfo>,
    pub subkeys: Vec<SubKeyInfo>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: DateTime<Utc>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_key: Option<String>,
}
