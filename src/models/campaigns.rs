use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CampaignState {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CampaignSort {
    Created,
    Updated,
    EndsAt,
    Published,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CampaignAlertStats {
    pub open_count: u64,
    pub closed_count: u64,
    pub in_progress_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Campaign {
    pub number: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub managers: Vec<SimpleUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_managers: Option<Vec<teams::Team>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
    pub ends_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    pub state: CampaignState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_stats: Option<CampaignAlertStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignCodeScanningAlert {
    pub repository_id: u64,
    pub alert_numbers: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCampaign {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_managers: Option<Vec<String>>,
    pub ends_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,
    pub code_scanning_alerts: Vec<CampaignCodeScanningAlert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_issues: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateCampaign {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_managers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
}
