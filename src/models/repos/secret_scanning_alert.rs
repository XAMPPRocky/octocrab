use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningAlert {
    pub number: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub url: Url,
    pub html_url: Url,
    pub locations_url: Url,
    pub state: State,
    pub resolution: Option<Resolution>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<SimpleUser>,
    pub secret_type: String,
    pub secret_type_display_name: String,
    pub secret: String,
    pub push_protection_bypassed_by: Option<SimpleUser>,
    pub push_protection_bypassed: Option<bool>,
    pub push_protection_bypassed_at: Option<DateTime<Utc>>,
    pub resolution_comment: Option<String>,
    pub validity: Validity,
    pub publicly_leaked: Option<bool>,
    pub multi_repo: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Resolved,
    Open,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resolution {
    FalsePositive,
    WontFix,
    Revoked,
    UsedInTests,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Validity {
    Active,
    Inactive,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSecretScanningAlert<'a> {
    pub state: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_comment: Option<&'a str>,
}
