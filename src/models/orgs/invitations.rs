use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{Author, InvitationId};

/// An invitation to an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgInvitation {
    pub id: InvitationId,
    #[serde(default)]
    pub login: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub failed_reason: Option<String>,
    #[serde(default)]
    pub inviter: Option<Author>,
    #[serde(default)]
    pub team_count: Option<u64>,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub invitation_teams_url: Option<String>,
}

/// A failed invitation to an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FailedOrgInvitation {
    pub id: InvitationId,
    #[serde(default)]
    pub login: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub failed_reason: Option<String>,
}
