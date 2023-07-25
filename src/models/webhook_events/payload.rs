mod branch_protection_rule;
mod check_run;
mod check_suite;
mod code_scanning_alert;
mod commit_comment;
mod create;
mod delete;
mod dependabot_alert;
mod deploy_key;
mod deployment;
mod deployment_protection_rule;
mod deployment_status;
mod discussion;
mod discussion_comment;
mod fork;
mod github_app_authorization;
mod gollum;
mod installation;
mod installation_repositories;
mod installation_target;
mod issue_comment;
mod issues;
mod label;
mod marketplace_purchase;
mod member;
mod membership;
mod merge_group;
mod meta;
mod milestone;
mod org_block;
mod organization;
mod package;
mod page_build;
mod personal_access_token_request;
mod ping;
mod project;
mod project_card;
mod project_column;
mod projects_v2;
mod projects_v2_item;
mod public;
mod pull_request;
mod pull_request_review;
mod pull_request_review_comment;
mod pull_request_review_thread;
mod push;
mod registry_package;
mod release;
mod repository;
mod repository_advisory;
mod repository_dispatch;
mod repository_import;
mod repository_vulnerability_alert;
mod secret_scanning_alert;
mod secret_scanning_alert_location;
mod security_advisory;
mod security_and_analysis;
mod sponsorship;
mod star;
mod status;
mod team;
mod team_add;
mod watch;
mod workflow_dispatch;
mod workflow_job;
mod workflow_run;

pub use self::{
    branch_protection_rule::*, check_run::*, check_suite::*, code_scanning_alert::*,
    commit_comment::*, create::*, delete::*, dependabot_alert::*, deploy_key::*, deployment::*,
    deployment_protection_rule::*, deployment_status::*, discussion::*, discussion_comment::*,
    fork::*, github_app_authorization::*, gollum::*, installation::*, installation_repositories::*,
    installation_target::*, issue_comment::*, issues::*, label::*, marketplace_purchase::*,
    member::*, membership::*, merge_group::*, meta::*, milestone::*, org_block::*, organization::*,
    package::*, page_build::*, personal_access_token_request::*, ping::*, project::*,
    project_card::*, project_column::*, projects_v2::*, projects_v2_item::*, public::*,
    pull_request::*, pull_request_review::*, pull_request_review_comment::*,
    pull_request_review_thread::*, push::*, registry_package::*, release::*, repository::*,
    repository_advisory::*, repository_dispatch::*, repository_import::*,
    repository_vulnerability_alert::*, secret_scanning_alert::*, secret_scanning_alert_location::*,
    security_advisory::*, security_and_analysis::*, sponsorship::*, star::*, status::*, team::*,
    team_add::*, watch::*, workflow_dispatch::*, workflow_job::*, workflow_run::*,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebhookEventPayload {
    BranchProtectionRuleWebhookEvent(Box<BranchProtectionRuleWebhookEventPayload>),
    CheckRunWebhookEvent(Box<CheckRunWebhookEventPayload>),
    CheckSuiteWebhookEvent(Box<CheckSuiteWebhookEventPayload>),
    CodeScanningAlertWebhookEvent(Box<CodeScanningAlertWebhookEventPayload>),
    CommitCommentWebhookEvent(Box<CommitCommentWebhookEventPayload>),
    CreateWebhookEvent(Box<CreateWebhookEventPayload>),
    DeleteWebhookEvent(Box<DeleteWebhookEventPayload>),
    DependabotAlertWebhookEvent(Box<DependabotAlertWebhookEventPayload>),
    DeployKeyWebhookEvent(Box<DeployKeyWebhookEventPayload>),
    DeploymentWebhookEvent(Box<DeploymentWebhookEventPayload>),
    DeploymentProtectionRuleWebhookEvent(Box<DeploymentProtectionRuleWebhookEventPayload>),
    DeploymentStatusWebhookEvent(Box<DeploymentStatusWebhookEventPayload>),
    DiscussionWebhookEvent(Box<DiscussionWebhookEventPayload>),
    DiscussionCommentWebhookEvent(Box<DiscussionCommentWebhookEventPayload>),
    ForkWebhookEvent(Box<ForkWebhookEventPayload>),
    GithubAppAuthorizationWebhookEvent(Box<GithubAppAuthorizationWebhookEventPayload>),
    GollumWebhookEvent(Box<GollumWebhookEventPayload>),
    InstallationWebhookEvent(Box<InstallationWebhookEventPayload>),
    InstallationRepositoriesWebhookEvent(Box<InstallationRepositoriesWebhookEventPayload>),
    InstallationTargetWebhookEvent(Box<InstallationTargetWebhookEventPayload>),
    IssueCommentWebhookEvent(Box<IssueCommentWebhookEventPayload>),
    IssuesWebhookEvent(Box<IssuesWebhookEventPayload>),
    LabelWebhookEvent(Box<LabelWebhookEventPayload>),
    MarketplacePurchaseWebhookEvent(Box<MarketplacePurchaseWebhookEventPayload>),
    MemberWebhookEvent(Box<MemberWebhookEventPayload>),
    MembershipWebhookEvent(Box<MembershipWebhookEventPayload>),
    MergeGroupWebhookEvent(Box<MergeGroupWebhookEventPayload>),
    MetaWebhookEvent(Box<MetaWebhookEventPayload>),
    MilestoneWebhookEvent(Box<MilestoneWebhookEventPayload>),
    OrgBlockWebhookEvent(Box<OrgBlockWebhookEventPayload>),
    OrganizationWebhookEvent(Box<OrganizationWebhookEventPayload>),
    PackageWebhookEvent(Box<PackageWebhookEventPayload>),
    PageBuildWebhookEvent(Box<PageBuildWebhookEventPayload>),
    PersonalAccessTokenRequestWebhookEvent(Box<PersonalAccessTokenRequestWebhookEventPayload>),
    PingWebhookEvent(Box<PingWebhookEventPayload>),
    ProjectCardWebhookEvent(Box<ProjectCardWebhookEventPayload>),
    ProjectWebhookEvent(Box<ProjectWebhookEventPayload>),
    ProjectColumnWebhookEvent(Box<ProjectColumnWebhookEventPayload>),
    ProjectsV2WebhookEvent(Box<ProjectsV2WebhookEventPayload>),
    ProjectsV2ItemWebhookEvent(Box<ProjectsV2ItemWebhookEventPayload>),
    PublicWebhookEvent(Box<PublicWebhookEventPayload>),
    PullRequestWebhookEvent(Box<PullRequestWebhookEventPayload>),
    PullRequestReviewWebhookEvent(Box<PullRequestReviewWebhookEventPayload>),
    PullRequestReviewCommentWebhookEvent(Box<PullRequestReviewCommentWebhookEventPayload>),
    PullRequestReviewThreadWebhookEvent(Box<PullRequestReviewThreadWebhookEventPayload>),
    PushWebhookEvent(Box<PushWebhookEventPayload>),
    RegistryPackageWebhookEvent(Box<RegistryPackageWebhookEventPayload>),
    ReleaseWebhookEvent(Box<ReleaseWebhookEventPayload>),
    RepositoryAdvisoryWebhookEvent(Box<RepositoryAdvisoryWebhookEventPayload>),
    RepositoryWebhookEvent(Box<RepositoryWebhookEventPayload>),
    RepositoryDispatchWebhookEvent(Box<RepositoryDispatchWebhookEventPayload>),
    RepositoryImportWebhookEvent(Box<RepositoryImportWebhookEventPayload>),
    RepositoryVulnerabilityAlertWebhookEvent(Box<RepositoryVulnerabilityAlertWebhookEventPayload>),
    SecretScanningAlertWebhookEvent(Box<SecretScanningAlertWebhookEventPayload>),
    SecretScanningAlertLocationWebhookEvent(Box<SecretScanningAlertLocationWebhookEventPayload>),
    SecurityAdvisoryWebhookEvent(Box<SecurityAdvisoryWebhookEventPayload>),
    SecurityAndAnalysisWebhookEvent(Box<SecurityAndAnalysisWebhookEventPayload>),
    SponsorshipWebhookEvent(Box<SponsorshipWebhookEventPayload>),
    StarWebhookEvent(Box<StarWebhookEventPayload>),
    StatusWebhookEvent(Box<StatusWebhookEventPayload>),
    TeamAddWebhookEvent(Box<TeamAddWebhookEventPayload>),
    TeamWebhookEvent(Box<TeamWebhookEventPayload>),
    WatchWebhookEvent(Box<WatchWebhookEventPayload>),
    WorkflowDispatchWebhookEvent(Box<WorkflowDispatchWebhookEventPayload>),
    WorkflowJobWebhookEvent(Box<WorkflowJobWebhookEventPayload>),
    WorkflowRunWebhookEvent(Box<WorkflowRunWebhookEventPayload>),
    UnknownWebhookEvent(Box<serde_json::Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RefType {
    Tag,
    Branch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PusherType {
    User,
    #[serde(untagged)]
    DeployKey(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MembershipScope {
    Team,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OldValue<T>
where
    T: Serialize,
    T: std::fmt::Debug + Clone + PartialEq,
{
    /// Old value, when the webhook payload is a change
    pub from: T,
}
