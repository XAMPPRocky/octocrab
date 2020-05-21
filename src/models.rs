//! Serde mappings from GitHub's JSON to structs.
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
    pub issue_url: Url,
    pub commits_url: Url,
    pub review_comments_url: Url,
    pub review_comment_url: Url,
    pub comments_url: Url,
    pub statuses_url: Url,
    pub number: i64,
    pub state: IssueState,
    pub locked: bool,
    pub title: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub requested_teams: Vec<RequestedTeam>,
    pub head: Head,
    pub base: Base,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    pub author_association: String,
    pub draft: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Organization {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub hooks_url: Url,
    pub issues_url: Url,
    pub members_url: Url,
    pub public_members_url: Url,
    pub avatar_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_organization_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_repository_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_repos: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_gists: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repository_settings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_requirement_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Issue {
    pub id: i64,
    pub node_id: String,
    pub url: Url,
    pub repository_url: Url,
    pub labels_url: Url,
    pub comments_url: Url,
    pub events_url: Url,
    pub html_url: Url,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub body: String,
    pub body_text: String,
    pub body_html: String,
    pub user: User,
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequestLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssueState {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestLink {
    pub url: Url,
    pub html_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MembershipInvitation {
    pub url: Url,
    pub state: String,
    pub role: String,
    pub organization_url: Url,
    pub organization: Organization,
    pub user: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    pub id: u64,
    pub node_id: String,
    pub url: Url,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    pub user: User,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ProjectCardContentType {
    Issue,
    PullRequest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectColumn {
    pub url: Url,
    pub project_url: Url,
    pub cards_url: Url,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Team {
    pub id: i64,
    pub node_id: String,
    pub url: Url,
    pub html_url: Url,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub privacy: String,
    pub permission: String,
    pub members_url: Url,
    pub repositories_url: Url,
    pub members_count: i64,
    pub repos_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub organization: Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plan {
    pub name: String,
    pub space: i64,
    pub private_repos: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Head {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Base {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Review {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ReviewState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestedReviewers {
    pub users: Vec<User>,
    pub teams: Vec<Team>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestedTeam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub members_url: Url,
    pub repositories_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Team>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub git_url: Url,
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
    pub ssh_url: String,
    pub stargazers_url: Url,
    pub statuses_url: Url,
    pub subscribers_url: Url,
    pub subscription_url: Url,
    pub tags_url: Url,
    pub teams_url: Url,
    pub trees_url: Url,
    pub clone_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<Url>,
    pub hooks_url: Url,
    pub svn_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<::serde_json::Value>,
    pub forks_count: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub size: u32,
    pub default_branch: String,
    pub open_issues_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub archived: bool,
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    pub author: User,
    pub committer: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {
    admin: bool,
    push: bool,
    pull: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRuns {
    pub total_count: i32,
    pub check_runs: Vec<CheckRun>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CheckStatus {
    Queued,
    Completed,
    InProgress,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CombinedStatus {
    pub state: StatusState,
    pub sha: String,
    pub total_count: i64,
    pub statuses: Vec<Status>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub state: StatusState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StatusState {
    Failure,
    Pending,
    Success,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ReviewState {
    Approved,
    Pending,
    ChangesRequested,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Links {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<SelfLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<HtmlLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_link: Option<IssueLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_link: Option<CommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments_link: Option<ReviewCommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment_link: Option<ReviewCommentLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_link: Option<CommitsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_link: Option<StatusesLink>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HtmlLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusesLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRepositories {
    pub total_count: i64,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InstallationPermissions {
    pub metadata: String,
    pub contents: String,
    pub issues: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Ref {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub node_id: String,
    pub url: Url,
    pub object: Object,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Object {
    Commit { sha: String, url: Url },
}

/// The author of a commit, identified by its name and email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUpdate {
    pub content: Content,
    pub commit: Commit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Content {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i64,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub r#type: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct PublicKey {
    pub key_id: String,
    pub key: String,
}
