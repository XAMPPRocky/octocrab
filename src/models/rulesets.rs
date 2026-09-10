use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for a GitHub ruleset.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(transparent)]
pub struct RulesetId(pub u64);

impl From<u64> for RulesetId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<RulesetId> for u64 {
    fn from(id: RulesetId) -> Self {
        id.0
    }
}

impl fmt::Display for RulesetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for RulesetId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Unique identifier for a GitHub rule suite evaluation.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(transparent)]
pub struct RuleSuiteId(pub u64);

impl From<u64> for RuleSuiteId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<RuleSuiteId> for u64 {
    fn from(id: RuleSuiteId) -> Self {
        id.0
    }
}

impl fmt::Display for RuleSuiteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for RuleSuiteId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Target of a ruleset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RulesetTarget {
    Branch,
    Tag,
    Push,
    Repository,
}

/// Level where the ruleset is configured.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RulesetSourceType {
    Repository,
    Organization,
    Enterprise,
    #[serde(other)]
    Other,
}

/// Enforcement status of a ruleset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RulesetEnforcement {
    Disabled,
    Active,
    Evaluate,
}

/// Type of actor that can bypass rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BypassActorType {
    Integration,
    OrganizationAdmin,
    RepositoryRole,
    Team,
    DeployKey,
    User,
    #[serde(other)]
    Other,
}

/// Mode in which an actor can bypass rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BypassMode {
    #[default]
    Always,
    PullRequest,
    Exempt,
}

/// Actor that can bypass the rules in a ruleset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RulesetBypassActor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<i64>,
    pub actor_type: BypassActorType,
    #[serde(default)]
    pub bypass_mode: BypassMode,
}

impl RulesetBypassActor {
    pub fn new(actor_type: BypassActorType) -> Self {
        Self {
            actor_id: None,
            actor_type,
            bypass_mode: BypassMode::Always,
        }
    }

    pub fn with_id(actor_type: BypassActorType, actor_id: i64) -> Self {
        Self {
            actor_id: Some(actor_id),
            actor_type,
            bypass_mode: BypassMode::Always,
        }
    }
}

/// Self and HTML links for a ruleset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RulesetLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<RulesetLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<RulesetLink>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RulesetLink {
    pub href: String,
}

/// Conditions for a ruleset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RulesetConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<RefNameCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<RepositoryNameCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<RepositoryIdCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_property: Option<RepositoryPropertyCondition>,
}

/// Ref name condition (e.g. branch or tag pattern).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RefNameCondition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
}

impl RefNameCondition {
    pub fn new(include: Vec<String>, exclude: Vec<String>) -> Self {
        Self { include, exclude }
    }
}

/// Repository name condition for organization rulesets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RepositoryNameCondition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}

/// Repository ID condition for organization rulesets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RepositoryIdCondition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository_ids: Vec<u64>,
}

/// Repository property condition for organization rulesets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RepositoryPropertyCondition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<PropertyTargetDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<PropertyTargetDefinition>,
}

/// Definition of custom properties targeting repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PropertyTargetDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// A rule within a ruleset.
// ponytail: Rule uses flexible parameters Value for forward compatibility with all present and future rule types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Rule {
    #[serde(rename = "type")]
    pub rule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl Rule {
    pub fn new(rule_type: impl Into<String>) -> Self {
        Self {
            rule_type: rule_type.into(),
            parameters: None,
        }
    }

    pub fn with_parameters(rule_type: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            rule_type: rule_type.into(),
            parameters: Some(parameters),
        }
    }
}

/// A GitHub repository or organization ruleset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ruleset {
    pub id: RulesetId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<RulesetTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<RulesetSourceType>,
    pub source: String,
    pub enforcement: RulesetEnforcement,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bypass_actors: Vec<RulesetBypassActor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_user_can_bypass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub links: Option<RulesetLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RulesetConditions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<Rule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Payload for creating or updating a ruleset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct UpdateRuleset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<RulesetTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<RulesetEnforcement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_actors: Option<Vec<RulesetBypassActor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RulesetConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

impl UpdateRuleset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn target(mut self, target: RulesetTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn enforcement(mut self, enforcement: RulesetEnforcement) -> Self {
        self.enforcement = Some(enforcement);
        self
    }

    pub fn bypass_actors(mut self, bypass_actors: Vec<RulesetBypassActor>) -> Self {
        self.bypass_actors = Some(bypass_actors);
        self
    }

    pub fn conditions(mut self, conditions: RulesetConditions) -> Self {
        self.conditions = Some(conditions);
        self
    }

    pub fn rules(mut self, rules: Vec<Rule>) -> Self {
        self.rules = Some(rules);
        self
    }
}

/// Result of a rule suite evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RuleSuiteResult {
    Pass,
    Fail,
    Bypass,
}

/// Summary item returned when listing rule suites.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RuleSuiteSummary {
    pub id: RuleSuiteId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name: Option<String>,
    pub before_sha: String,
    pub after_sha: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub repository_id: u64,
    pub repository_name: String,
    pub pushed_at: DateTime<Utc>,
    pub result: RuleSuiteResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<RuleSuiteResult>,
}

/// Detailed rule suite evaluation result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RuleSuite {
    pub id: RuleSuiteId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name: Option<String>,
    pub before_sha: String,
    pub after_sha: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub repository_id: u64,
    pub repository_name: String,
    pub pushed_at: DateTime<Utc>,
    pub result: RuleSuiteResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<RuleSuiteResult>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_evaluations: Vec<RuleEvaluation>,
}

/// An evaluation of a single rule in a rule suite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RuleEvaluation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_source: Option<RuleEvaluationSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<String>,
    pub result: RuleEvaluationResult,
    pub rule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// Result of evaluating an individual rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RuleEvaluationResult {
    Pass,
    Fail,
}

/// Source of a rule evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RuleEvaluationSource {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A rule evaluated for a branch from `/repos/{owner}/{repo}/rules/branches/{branch}`.
// ponytail: RepoRule uses flattened additional_properties for rule parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoRule {
    #[serde(rename = "type")]
    pub rule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_source_type: Option<RulesetSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_id: Option<RulesetId>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
