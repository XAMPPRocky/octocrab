use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningAlert {
    pub number: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
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
    #[serde(default)]
    pub repository: Option<Repository>,
    pub push_protection_bypassed_by: Option<SimpleUser>,
    pub push_protection_bypassed: Option<bool>,
    pub push_protection_bypassed_at: Option<DateTime<Utc>>,
    pub resolution_comment: Option<String>,
    pub validity: Validity,
    pub publicly_leaked: Option<bool>,
    pub multi_repo: Option<bool>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub provider_slug: Option<String>,
    #[serde(default)]
    pub is_base64_encoded: Option<bool>,
    #[serde(default)]
    pub has_more_locations: Option<bool>,
    #[serde(default)]
    pub assigned_to: Option<SimpleUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum State {
    Resolved,
    Open,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Resolution {
    FalsePositive,
    WontFix,
    Revoked,
    UsedInTests,
    PatternEdited,
    PatternDeleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Validity {
    Active,
    Inactive,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct UpdateSecretScanningAlert<'a> {
    pub state: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_comment: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<&'a str>,
}

impl<'a> UpdateSecretScanningAlert<'a> {
    pub fn new(state: &'a str) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SecretsScanningAlertLocation {
    Commit {
        path: String,
        start_line: u32,
        end_line: u32,
        start_column: u32,
        end_column: u32,
        blob_sha: String,
        blob_url: String,
        commit_sha: String,
        commit_url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        html_url: Option<Url>,
    },
    WikiCommit {
        path: String,
        start_line: u32,
        end_line: u32,
        start_column: u32,
        end_column: u32,
        blob_sha: String,
        page_url: String,
        commit_sha: String,
        commit_url: String,
    },
    IssueTitle {
        issue_title_url: String,
    },
    IssueBody {
        issue_body_url: String,
    },
    IssueComment {
        issue_comment_url: String,
    },
    DiscussionTitle {
        discussion_title_url: String,
    },
    DiscussionBody {
        discussion_body_url: String,
    },
    DiscussionComment {
        discussion_comment_url: String,
    },
    PullRequestTitle {
        pull_request_title_url: String,
    },
    PullRequestBody {
        pull_request_body_url: String,
    },
    PullRequestComment {
        pull_request_comment_url: String,
    },
    PullRequestReview {
        pull_request_review_url: String,
    },
    PullRequestReviewComment {
        pull_request_review_comment_url: String,
    },
}

/// A secret scanning custom pattern for an organization or repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningCustomPattern {
    pub id: u64,
    pub name: String,
    pub pattern: String,
    pub slug: String,
    pub state: String,
    pub push_protection_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_match: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_not_match: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_pattern_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Request entry for bulk creating secret scanning custom patterns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateSecretScanningCustomPattern {
    pub name: String,
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_protection_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_match: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_not_match: Option<Vec<String>>,
}

impl CreateSecretScanningCustomPattern {
    pub fn new(name: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pattern: pattern.into(),
            ..Default::default()
        }
    }
}

/// Response from bulk creating secret scanning custom patterns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkCreatedCustomPatterns {
    pub created_patterns: Vec<SecretScanningCustomPattern>,
}

/// Request entry for bulk deleting secret scanning custom patterns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteSecretScanningCustomPattern {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_pattern_version: Option<String>,
}

impl DeleteSecretScanningCustomPattern {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            custom_pattern_version: None,
        }
    }
}

/// Request body for updating a secret scanning custom pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSecretScanningCustomPattern {
    pub custom_pattern_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_protection_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_match: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_not_match: Option<Vec<String>>,
}

impl UpdateSecretScanningCustomPattern {
    pub fn new(custom_pattern_version: Option<String>) -> Self {
        Self {
            custom_pattern_version,
            ..Default::default()
        }
    }
}

/// Organization secret scanning pattern configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningPatternConfigurations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_config_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_pattern_overrides: Option<Vec<ProviderPatternOverride>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_pattern_overrides: Option<Vec<CustomPatternOverride>>,
}

/// Provider pattern override configuration in an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProviderPatternOverride {
    pub token_type: String,
    pub slug: String,
    pub display_name: String,
    pub alert_total: u64,
    pub alert_total_percentage: u64,
    pub false_positives: u64,
    pub false_positive_rate: u64,
    pub bypass_rate: u64,
    pub default_setting: String,
    pub setting: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_setting: Option<String>,
}

/// Custom pattern override configuration in an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomPatternOverride {
    pub token_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_pattern_version: Option<String>,
    pub slug: String,
    pub display_name: String,
    pub alert_total: u64,
    pub alert_total_percentage: u64,
    pub false_positives: u64,
    pub false_positive_rate: u64,
    pub bypass_rate: u64,
    pub default_setting: String,
    pub setting: String,
}

/// Request body for updating organization secret scanning pattern configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSecretScanningPatternConfigurations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_config_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pattern_settings: Option<Vec<ProviderPatternSetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_pattern_settings: Option<Vec<CustomPatternSetting>>,
}

/// Provider pattern setting update entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderPatternSetting {
    pub token_type: String,
    pub push_protection_setting: String,
}

/// Custom pattern setting update entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPatternSetting {
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_pattern_version: Option<String>,
    pub push_protection_setting: String,
}

/// Response from updating organization secret scanning pattern configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdatedSecretScanningPatternConfigurations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_config_version: Option<String>,
}

/// Reason for bypassing secret scanning push protection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PushProtectionBypassReason {
    FalsePositive,
    UsedInTests,
    WillFixLater,
}

/// Push protection bypass response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PushProtectionBypass {
    pub reason: PushProtectionBypassReason,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<DateTime<Utc>>,
    pub token_type: String,
}

/// Secret scanning scan history for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningScanHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental_scans: Option<Vec<SecretScanningScan>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backfill_scans: Option<Vec<SecretScanningScan>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_update_scans: Option<Vec<SecretScanningScan>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_pattern_backfill_scans: Option<Vec<CustomPatternBackfillScan>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic_secrets_backfill_scans: Option<Vec<SecretScanningScan>>,
}

/// A secret scanning scan entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningScan {
    #[serde(rename = "type")]
    pub scan_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

/// A custom pattern backfill scan entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomPatternBackfillScan {
    #[serde(rename = "type")]
    pub scan_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_scope: Option<String>,
}
