//! Serde mappings from GitHub's JSON to structs.
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub mod activity;
pub mod events;
pub mod issues;
pub mod orgs;
pub mod pulls;
pub mod repos;
pub mod teams;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Contents {
    #[serde(rename = "type")]
    pub contents_type: String,
    pub encoding: String,
    pub size: u64,
    pub name: String,
    pub path: String,
    pub content: String,
    pub sha: String,
    pub url: Url,
    pub git_url: Url,
    pub html_url: Url,
    pub download_url: Url,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Event {
    AddedToProject,
    Assigned,
    Closed,
    ConvertedNoteToIssue,
    Demilestoned,
    HeadRefDeleted,
    HeadRefForcePushed,
    HeadRefRestored,
    Labeled,
    Locked,
    Mentioned,
    MarkedAsDuplicate,
    Merged,
    Milestoned,
    MovedColumnsInProject,
    Referenced,
    RemovedFromProject,
    Renamed,
    Reopened,
    ReviewDismissed,
    ReviewRequested,
    ReviewRequestRemoved,
    Subscribed,
    Transferred,
    Unassigned,
    Unlabeled,
    Unlocked,
    UnmarkedAsDuplicate,
    UserBlocked,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssueState {
    Open,
    Closed,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub actor: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_card: Option<ProjectCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectCard {
    pub id: u64,
    pub url: Url,
    pub project_id: i64,
    pub project_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_column_name: Option<String>,
    pub column_url: Url,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Project {
    pub owner_url: Url,
    pub url: Url,
    pub html_url: Url,
    pub columns_url: Url,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub creator: User,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ProjectCardContentType {
    Issue,
    PullRequest,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectColumn {
    pub url: Url,
    pub project_url: Url,
    pub cards_url: Url,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuePullRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
    pub url: Url,
    pub html_url: Url,
    pub followers_url: Url,
    pub following_url: Url,
    pub gists_url: Url,
    pub starred_url: Url,
    pub subscriptions_url: Url,
    pub organizations_url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub received_events_url: Url,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Milestone {
    pub url: Url,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<Url>,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_issues: Option<i64>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub fork: bool,
    pub url: Url,
    pub archive_url: Url,
    pub assignees_url: Url,
    pub blobs_url: Url,
    pub branches_url: Url,
    pub collaborators_url: Url,
    pub comments_url: Url,
    pub commits_url: Url,
    pub compare_url: Url,
    pub contents_url: Url,
    pub contributors_url: Url,
    pub deployments_url: Url,
    pub downloads_url: Url,
    pub events_url: Url,
    pub forks_url: Url,
    pub git_commits_url: Url,
    pub git_refs_url: Url,
    pub git_tags_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_url: Option<Url>,
    pub issue_comment_url: Url,
    pub issue_events_url: Url,
    pub issues_url: Url,
    pub keys_url: Url,
    pub labels_url: Url,
    pub languages_url: Url,
    pub merges_url: Url,
    pub milestones_url: Url,
    pub notifications_url: Url,
    pub pulls_url: Url,
    pub releases_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    pub stargazers_url: Url,
    pub statuses_url: Url,
    pub subscribers_url: Url,
    pub subscription_url: Url,
    pub tags_url: Url,
    pub teams_url: Url,
    pub trees_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<Url>,
    pub hooks_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svn_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    pub html_url: Option<Url>,
    pub description: Option<String>,
    pub implementation: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
    pub body: Option<String>,
    pub featured: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Code {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: Url,
    pub git_url: Url,
    pub html_url: Url,
    pub repository: Repository,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {
    admin: bool,
    push: bool,
    pull: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRuns {
    pub total_count: i32,
    pub check_runs: Vec<CheckRun>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CheckStatus {
    Queued,
    Completed,
    InProgress,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CombinedStatus {
    pub state: StatusState,
    pub sha: String,
    pub total_count: i64,
    pub statuses: Vec<Status>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub state: StatusState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StatusState {
    Failure,
    Pending,
    Success,
    Error,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRepositories {
    pub total_count: i64,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Installation {
    pub id: i64,
    pub account: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tokens_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    pub permissions: InstallationPermissions,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InstallationPermissions {
    pub metadata: String,
    pub contents: String,
    pub issues: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InstallationToken {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub permissions: Permissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct PublicKey {
    pub key_id: String,
    pub key: String,
}
