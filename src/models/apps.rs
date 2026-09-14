use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct App {
    pub id: AppId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub node_id: String,
    pub owner: Author,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub external_url: Url,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub permissions: Permissions,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installations_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_references: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gists: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_limits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_events: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_secrets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_events: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starring: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watching: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<String>,
}

/// A pending installation request for a GitHub App.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#list-installation-requests-for-the-authenticated-app)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRequest {
    pub id: InstallationRequestId,
    pub node_id: String,
    pub account: Author,
    pub requester: Author,
    pub created_at: DateTime<Utc>,
}

/// Repositories accessible to an installation.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#list-repositories-accessible-to-the-app-installation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRepositories {
    pub total_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<String>,
    pub repositories: Vec<Repository>,
}

/// Request body for creating an installation access token.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-an-installation-access-token-for-an-app)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateInstallationAccessToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_ids: Option<Vec<RepositoryId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashMap<String, String>>,
}

/// Information about an OAuth application within an authorization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AppAuthorizationApp {
    pub url: Url,
    pub name: String,
    pub client_id: String,
}

/// Installation information associated with a scoped access token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AppAuthorizationInstallation {
    #[serde(default)]
    pub permissions: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<Url>,
    pub account: Author,
}

/// An OAuth application authorization or token.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/oauth-applications?apiVersion=2022-11-28#check-a-token)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AppAuthorization {
    pub id: u64,
    pub url: Url,
    pub scopes: Vec<String>,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_last_eight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_token: Option<String>,
    pub app: AppAuthorizationApp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation: Option<AppAuthorizationInstallation>,
}

/// Request body for creating a scoped access token.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-a-scoped-access-token)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScopedAccessToken {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_ids: Option<Vec<u64>>,
}

/// Request body for updating webhook configuration.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/webhooks?apiVersion=2022-11-28#update-a-webhook-configuration-for-an-app)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<String>,
}

/// Request body for token operations (check, reset, delete).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenBody {
    pub access_token: String,
}
