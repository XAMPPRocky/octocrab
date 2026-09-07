use serde::{Deserialize, Serialize};

/// Hypermedia links to resources accessible in GitHub's REST API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApiRoot {
    pub current_user_url: String,
    pub current_user_authorizations_html_url: String,
    pub authorizations_url: String,
    pub code_search_url: String,
    pub commit_search_url: String,
    pub emails_url: String,
    pub emojis_url: String,
    pub events_url: String,
    pub feeds_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_url: Option<String>,
    pub issue_search_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub label_search_url: String,
    pub notifications_url: String,
    pub organization_url: String,
    pub organization_repositories_url: String,
    pub organization_teams_url: String,
    pub public_gists_url: String,
    pub rate_limit_url: String,
    pub repository_url: String,
    pub repository_search_url: String,
    pub current_user_repositories_url: String,
    pub starred_url: String,
    pub starred_gists_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_search_url: Option<String>,
    pub user_url: String,
    pub user_organizations_url: String,
    pub user_repositories_url: String,
    pub user_search_url: String,
}

/// Meta information about GitHub, including lists of IP addresses and domain names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Meta {
    pub verifiable_password_authentication: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_fingerprints: Option<SshKeyFingerprints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_enterprise_importer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_macos: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependabot: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copilot: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_signing_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Domains>,
}

/// SSH key fingerprints for GitHub's public keys.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SshKeyFingerprints {
    #[serde(rename = "SHA256_RSA", skip_serializing_if = "Option::is_none")]
    pub sha256_rsa: Option<String>,
    #[serde(rename = "SHA256_DSA", skip_serializing_if = "Option::is_none")]
    pub sha256_dsa: Option<String>,
    #[serde(rename = "SHA256_ECDSA", skip_serializing_if = "Option::is_none")]
    pub sha256_ecdsa: Option<String>,
    #[serde(rename = "SHA256_ED25519", skip_serializing_if = "Option::is_none")]
    pub sha256_ed25519: Option<String>,
}

/// Domains used by GitHub services.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Domains {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codespaces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copilot: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
}
