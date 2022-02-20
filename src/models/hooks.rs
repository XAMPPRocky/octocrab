use super::*;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hook {
    pub r#type: String,
    pub id: u64,
    pub name: String,
    pub events: Vec<String>,
    pub config: Config,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliveries_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
