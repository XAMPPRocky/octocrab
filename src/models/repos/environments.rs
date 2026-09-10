use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{AppId, BranchPolicyId, EnvironmentId, ProtectionRuleId, TeamId, UserId};

/// A repository environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Environment {
    pub id: EnvironmentId,
    pub node_id: String,
    pub name: String,
    pub url: Url,
    pub html_url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_admins_bypass: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protection_rules: Vec<EnvironmentProtectionRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicySettings>,
}

/// Response returned by the list environments endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Environments {
    pub total_count: u64,
    pub environments: Vec<Environment>,
}

/// A protection rule configured on an environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentProtectionRule {
    pub id: u64,
    pub node_id: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_timer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_self_review: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<EnvironmentProtectionRuleReviewer>>,
}

/// A reviewer entry in an environment's protection rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentProtectionRuleReviewer {
    #[serde(rename = "type")]
    pub reviewer_type: Option<String>,
    pub reviewer: Option<serde_json::Value>,
}

/// Deployment branch policy configuration settings for an environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentBranchPolicySettings {
    pub protected_branches: bool,
    pub custom_branch_policies: bool,
}

impl DeploymentBranchPolicySettings {
    pub fn new(protected_branches: bool, custom_branch_policies: bool) -> Self {
        Self {
            protected_branches,
            custom_branch_policies,
        }
    }
}

/// A reviewer to configure on an environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnvironmentReviewer {
    #[serde(rename = "type")]
    pub reviewer_type: ReviewerType,
    pub id: u64,
}

impl EnvironmentReviewer {
    pub fn user(user_id: impl Into<UserId>) -> Self {
        Self {
            reviewer_type: ReviewerType::User,
            id: user_id.into().0,
        }
    }

    pub fn team(team_id: impl Into<TeamId>) -> Self {
        Self {
            reviewer_type: ReviewerType::Team,
            id: team_id.into().0,
        }
    }
}

/// The type of reviewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewerType {
    User,
    Team,
}

/// A deployment branch or tag policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentBranchPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BranchPolicyId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<BranchPolicyType>,
}

/// Response returned by the list deployment branch policies endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentBranchPolicies {
    pub total_count: u64,
    pub branch_policies: Vec<DeploymentBranchPolicy>,
}

/// The type of deployment branch policy target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BranchPolicyType {
    Branch,
    Tag,
}

/// A custom deployment protection rule configured on an environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomDeploymentProtectionRule {
    pub id: ProtectionRuleId,
    pub node_id: String,
    pub enabled: bool,
    pub app: CustomDeploymentRuleApp,
}

/// Response returned by the list custom deployment protection rules endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomDeploymentProtectionRules {
    pub total_count: u64,
    pub custom_deployment_protection_rules: Vec<CustomDeploymentProtectionRule>,
}

/// GitHub App implementing a custom deployment protection rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomDeploymentRuleApp {
    pub id: AppId,
    pub slug: String,
    pub integration_url: Url,
    pub node_id: String,
}

/// Response returned by the list available custom deployment protection rule apps endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomDeploymentRuleApps {
    pub total_count: u64,
    pub available_custom_deployment_protection_rule_integrations: Vec<CustomDeploymentRuleApp>,
}
