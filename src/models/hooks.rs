use super::{webhook_events::WebhookEventType, *};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hook {
    pub r#type: String,
    pub active: bool,
    /// Only included in webhook payload received by GitHub Apps. When you
    /// register a new GitHub App, GitHub sends a ping event to the webhook URL
    /// you specified during registration. The GitHub App ID sent in this field
    /// is required for authenticating an app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<AppId>,
    pub id: u64,
    /// The type of webhook. At the time of writing, the only valid value is
    /// 'web'
    pub name: String,
    pub events: Vec<WebhookEventType>,
    pub config: Config,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_response: Option<LastResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliveries_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LastResponse {
    pub code: Option<i64>,
    pub status: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Json,
    #[default]
    Form,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Delivery {
    pub id: HookDeliveryId,
    pub guid: String,
    pub delivered_at: DateTime<Utc>,
    pub duration: f64,
    pub status: String,
    pub status_code: usize,
    pub event: Option<WebhookEventType>,
    pub action: Option<String>,
    pub installation_id: Option<InstallationId>,
    pub repository_id: Option<InstallationId>,
    pub redelivery: bool,
}

/// Detailed information about a webhook delivery, including request and response payloads.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#get-a-delivery-for-a-repository-webhook)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeliveryDetail {
    pub id: HookDeliveryId,
    pub guid: String,
    pub delivered_at: DateTime<Utc>,
    pub redelivery: bool,
    pub duration: f64,
    pub status: String,
    pub status_code: usize,
    pub event: String,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub installation_id: Option<InstallationId>,
    #[serde(default)]
    pub repository_id: Option<RepositoryId>,
    #[serde(default)]
    pub throttled_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub url: Option<String>,
    pub request: DeliveryRequest,
    pub response: DeliveryResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeliveryRequest {
    #[serde(default)]
    pub headers: Option<serde_json::Value>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeliveryResponse {
    #[serde(default)]
    pub headers: Option<serde_json::Value>,
    #[serde(default)]
    pub payload: Option<String>,
}

/// Parameters for updating a webhook configuration.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateHookConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<String>,
}
