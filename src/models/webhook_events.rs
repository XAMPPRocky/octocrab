mod payload;

use super::{orgs::Organization, Author, Installation, InstallationId, Repository, RepositoryId};
use serde::{Deserialize, Serialize};

pub use payload::WebhookEventPayload;

/// A GitHub event.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WebhookEvent {
    pub sender: Option<Author>,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub installation: Option<EventInstallation>,
    #[serde(skip)]
    pub kind: WebhookEventType,
    #[serde(flatten)]
    pub specific: WebhookEventPayload,
}

impl WebhookEvent {
    /// Deserialize the body of a webhook event according to the category in the header of the request.
    ///
    /// ```
    /// use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
    ///
    /// let json = r#"{
    ///   "zen": "Design for failure.",
    ///   "hook_id": 423885699,
    ///   "hook": {
    ///     "type": "App",
    ///     "id": 423885699,
    ///     "name": "web",
    ///     "active": true,
    ///     "events": [
    ///       "issues",
    ///       "issue_comment",
    ///       "meta",
    ///       "pull_request",
    ///       "pull_request_review",
    ///       "pull_request_review_comment",
    ///       "pull_request_review_thread",
    ///       "repository"
    ///     ],
    ///     "config": {
    ///       "content_type": "json",
    ///       "insecure_ssl": "0",
    ///       "secret": "********",
    ///       "url": "https://smee.io/R"
    ///     },
    ///     "updated_at": "2023-07-13T09:30:45Z",
    ///     "created_at": "2023-07-13T09:30:45Z",
    ///     "app_id": 360617,
    ///     "deliveries_url": "https://api.github.com/app/hook/deliveries"
    ///   }
    /// }"#;
    ///
    /// // Value picked from the `X-GitHub-Event` header
    /// let event_name = "ping";
    ///
    /// let event = WebhookEvent::try_from_header_and_body(event_name, json)?;
    /// assert_eq!(event.kind, WebhookEventType::Ping);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn try_from_header_and_body<B>(header: &str, body: &B) -> Result<Self, serde_json::Error>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let kind = if header.starts_with('"') {
            serde_json::from_str::<WebhookEventType>(header)?
        } else {
            serde_json::from_str::<WebhookEventType>(&format!("\"{header}\""))?
        };

        // Intermediate structure allows to separate the common fields from
        // the event specific one.
        #[derive(Deserialize)]
        struct Intermediate {
            sender: Option<Author>,
            repository: Option<Repository>,
            organization: Option<Organization>,
            installation: Option<EventInstallation>,
            #[serde(flatten)]
            specific: serde_json::Value,
        }

        let Intermediate {
            sender,
            repository,
            organization,
            installation,
            specific,
        } = serde_json::from_slice::<Intermediate>(body.as_ref())?;

        let specific = kind.parse_specific_payload(specific)?;

        Ok(Self {
            sender,
            repository,
            organization,
            installation,
            kind,
            specific,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// This event occurs when there is activity relating to branch protection rules. For more information, see "About protected branches." For information about the APIs to manage branch protection rules, see the GraphQL documentation or "Branch protection" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Administration" repository permission
    BranchProtectionRule,
    /// This event occurs when there is activity relating to a check run. For information about check runs, see "Getting started with the Checks API." For information about the APIs to manage check runs, see the GraphQL API documentation or "Check Runs" in the REST API documentation.
    ///
    /// For activity relating to check suites, use the check-suite event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Checks" repository permission. To receive the rerequested and requested_action event types, the app must have at least write-level access for the "Checks" permission. GitHub Apps with write-level access for the "Checks" permission are automatically subscribed to this webhook event.
    ///
    /// Repository and organization webhooks only receive payloads for the created and completed event types in repositories.
    ///
    /// Note: The API only looks for pushes in the repository where the check run was created. Pushes to a branch in a forked repository are not detected and return an empty pull_requests array and a null value for head_branch.
    CheckRun,
    /// This event occurs when there is activity relating to a check suite. For information about check suites, see "Getting started with the Checks API." For information about the APIs to manage check suites, see the GraphQL API documentation or "Check Suites" in the REST API documentation.
    ///
    /// For activity relating to check runs, use the check_run event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Checks" permission. To receive the requested and rerequested event types, the app must have at lease write-level access for the "Checks" permission. GitHub Apps with write-level access for the "Checks" permission are automatically subscribed to this webhook event.
    ///
    /// Repository and organization webhooks only receive payloads for the completed event types in repositories.
    ///
    /// Note: The API only looks for pushes in the repository where the check suite was created. Pushes to a branch in a forked repository are not detected and return an empty pull_requests array and a null value for head_branch.
    CheckSuite,
    /// This event occurs when there is activity relating to code scanning alerts in a repository. For more information, see "About code scanning" and "About code scanning alerts." For information about the API to manage code scanning, see "Code scanning" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Code scanning alerts" repository permission.
    CodeScanningAlert,
    /// This event occurs when there is activity relating to commit comments. For more information about commit comments, see "Commenting on a pull request." For information about the APIs to manage commit comments, see the GraphQL API documentation or "Commit comments" in the REST API documentation.
    ///
    /// For activity relating to comments on pull request reviews, use the pull_request_review_comment event. For activity relating to issue comments, use the issue_comment event. For activity relating to discussion comments, use the discussion_comment event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    CommitComment,
    /// This event occurs when a Git branch or tag is created.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    ///
    /// Note: This event will not occur when more than three tags are created at once.
    Create,
    /// This event occurs when a Git branch or tag is deleted.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    ///
    /// Note: This event will not occur when more than three tags are deleted at once.
    Delete,
    /// This event occurs when there is activity relating to Dependabot alerts.
    ///
    /// For more information about Dependabot alerts, see "About Dependabot alerts." For information about the API to manage Dependabot alerts, see "Dependabot alerts" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Dependabot alerts" repository permission.
    ///
    /// Note: Webhook events for Dependabot alerts are currently in beta and subject to change.
    DependabotAlert,
    /// This event occurs when there is activity relating to deploy keys. For more information, see "Managing deploy keys." For information about the APIs to manage deploy keys, see the GraphQL API documentation or "Deploy keys" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Deployments" repository permission.
    DeployKey,
    /// This event occurs when there is activity relating to deployments. For more information, see "About deployments." For information about the APIs to manage deployments, see the GraphQL API documentation or "Deployments" in the REST API documentation.
    ///
    /// For activity relating to deployment status, use the deployment_status event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Deployments" repository permission.
    Deployment,
    /// This event occurs when there is activity relating to deployment protection rules. For more information, see "Using environments for deployment." For information about the API to manage deployment protection rules, see the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Deployments" repository permission.
    DeploymentProtectionRule,
    /// This event occurs when there is activity relating to deployment statuses. For more information, see "About deployments." For information about the APIs to manage deployments, see the GraphQL API documentation or "Deployments" in the REST API documentation.
    ///
    /// For activity relating to deployment creation, use the deployment event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Deployments" repository permission.
    DeploymentStatus,
    /// This event occurs when there is activity relating to a discussion. For more information about discussions, see "GitHub Discussions." For information about the API to manage discussions, see the GraphQL documentation.
    ///
    /// For activity relating to a comment on a discussion, use the discussion_comment event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Discussions" repository permission.
    ///
    /// Note: Webhook events for GitHub Discussions are currently in beta and subject to change.
    Discussion,
    /// This event occurs when there is activity relating to a comment on a discussion. For more information about discussions, see "GitHub Discussions." For information about the API to manage discussions, see the GraphQL documentation.
    ///
    /// For activity relating to a discussion as opposed to comments on a discussion, use the discussion event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Discussions" repository permission.
    ///
    /// Note: Webhook events for GitHub Discussions are currently in beta and subject to change.
    DiscussionComment,
    /// This event occurs when someone forks a repository. For more information, see "Fork a repo." For information about the API to manage forks, see "Forks" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    Fork,
    /// This event occurs when a user revokes their authorization of a GitHub App. For more information, see "About apps." For information about the API to manage GitHub Apps, see the GraphQL API documentation or "Apps" in the REST API documentation.
    ///
    /// A GitHub App receives this webhook by default and cannot unsubscribe from this event.
    ///
    /// Anyone can revoke their authorization of a GitHub App from their GitHub account settings page. Revoking the authorization of a GitHub App does not uninstall the GitHub App. You should program your GitHub App so that when it receives this webhook, it stops calling the API on behalf of the person who revoked the token. If your GitHub App continues to use a revoked access token, it will receive the 401 Bad Credentials error. For details about requests with a user access token, which require GitHub App authorization, see "Authenticating with a GitHub App on behalf of a user."
    GithubAppAuthorization,
    /// This event occurs when someone creates or updates a wiki page. For more information, see "About wikis."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    Gollum,
    /// This event occurs when there is activity relating to a GitHub App installation. All GitHub Apps receive this event by default. You cannot manually subscribe to this event.
    ///
    /// For more information about GitHub Apps, see "About apps." For information about the APIs to manage GitHub Apps, see the GraphQL API documentation or "Apps" in the REST API documentation.
    Installation,
    /// This event occurs when there is activity relating to which repositories a GitHub App installation can access. All GitHub Apps receive this event by default. You cannot manually subscribe to this event.
    ///
    /// For more information about GitHub Apps, see "About apps." For information about the APIs to manage GitHub Apps, see the GraphQL API documentation or "Apps" in the REST API documentation.
    InstallationRepositories,
    /// This event occurs when there is activity relating to the user or organization account that a GitHub App is installed on. For more information, see "About apps." For information about the APIs to manage GitHub Apps, see the GraphQL API documentation or "Apps" in the REST API documentation.
    InstallationTarget,
    /// This event occurs when there is activity relating to a comment on an issue or pull request. For more information about issues and pull requests, see "About issues" and "About pull requests." For information about the APIs to manage issue comments, see the GraphQL documentation or "Issue comments" in the REST API documentation.
    ///
    /// For activity relating to an issue as opposed to comments on an issue, use the issue event. For activity related to pull request reviews or pull request review comments, use the pull_request_review or pull_request_review_comment events. For more information about the different types of pull request comments, see "Working with comments."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Issues" repository permission.
    IssueComment,
    /// This event occurs when there is activity relating to an issue. For more information about issues, see "About issues." For information about the APIs to manage issues, see the GraphQL documentation or "Issues" in the REST API documentation.
    ///
    /// For activity relating to a comment on an issue, use the issue_comment event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Issues" repository permission.
    Issues,
    /// This event occurs when there is activity relating to labels. For more information, see "Managing labels." For information about the APIs to manage labels, see the GraphQL documentation or "Labels" in the REST API documentation.
    ///
    /// If you want to receive an event when a label is added to or removed from an issue, pull request, or discussion, use the labeled or unlabeled action type for the issues, pull_request, or discussion events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Metadata" repository permission.
    Label,
    /// This event occurs when there is activity relating to a GitHub Marketplace purchase. For more information, see "GitHub Marketplace." For information about the APIs to manage GitHub Marketplace listings, see the GraphQL documentation or "GitHub Marketplace" in the REST API documentation.
    MarketplacePurchase,
    /// This event occurs when there is activity relating to collaborators in a repository. For more information, see "Adding outside collaborators to repositories in your organization." For more information about the API to manage repository collaborators, see the GraphQL API documentation or "Collaborators" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Members" organization permission.
    Member,
    /// This event occurs when there is activity relating to team membership. For more information, see "About teams." For more information about the APIs to manage team memberships, see the GraphQL API documentation or "Team members" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Members" organization permission.
    Membership,
    /// This event occurs when there is activity relating to a merge group in a merge queue. For more information, see "Managing a merge queue."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Merge queues" repository permission.
    MergeGroup,
    /// This event occurs when there is activity relating to a webhook itself.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Meta" app permission.
    Meta,
    /// This event occurs when there is activity relating to milestones. For more information, see "About milestones." For information about the APIs to manage milestones, see the GraphQL documentation or "Milestones" in the REST API documentation.
    ///
    /// If you want to receive an event when an issue or pull request is added to or removed from a milestone, use the milestoned or demilestoned action type for the issues or pull_request events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Issues" or "Pull requests" repository permissions.
    Milestone,
    /// This event occurs when organization owners or moderators block or unblock a non-member from collaborating on the organization's repositories. For more information, see "Blocking a user from your organization." For information about the APIs to manage blocked users, see the GraphQL documentation or "Blocking users" in the REST API documentation.
    ///
    /// If you want to receive an event when members are added or removed from an organization, use the organization event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Administration" organization permission.
    OrgBlock,
    /// This event occurs when there is activity relating to an organization and its members. For more information, see "About organizations." For information about the APIs to manage organizations, see the GraphQL documentation or "Organizations" in the REST API documentation.
    ///
    /// If you want to receive an event when a non-member is blocked or unblocked from an organization, use the org_block event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Members" organization permission.
    Organization,
    /// This event occurs when there is activity relating to GitHub Packages. For more information, see "Introduction to GitHub Packages." For information about the APIs to manage GitHub Packages, see the GraphQL API documentation or "Packages" in the REST API documentation.
    ///
    /// To install this event on a GitHub App, the app must have at least read-level access for the "Packages" repository permission.
    Package,
    /// This event occurs when there is an attempted build of a GitHub Pages site. This event occurs regardless of whether the build is successful. For more information, see "Configuring a publishing source for your GitHub Pages site." For information about the API to manage GitHub Pages, see "Pages" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pages" repository permission.
    PageBuild,
    /// This event occurs when there is activity relating to a request for a fine-grained personal access token to access resources that belong to a resource owner that requires approval for token access. For more information, see "Creating a personal access token."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Personal access token requests" organization permission.
    ///
    /// Note: Fine-grained PATs are in public beta. Related APIs, events, and functionality are subject to change.
    PersonalAccessTokenRequest,
    /// This event occurs when you create a new webhook. The ping event is a confirmation from GitHub that you configured the webhook correctly.
    Ping,
    /// This event occurs when there is activity relating to a card on a classic project. For more information, see "About projects (classic)." For information about the API to manage classic projects, see the GraphQL API documentation or "Projects (classic)" in the REST API documentation.
    ///
    /// For activity relating to a project or a column on a project, use the project and project_column event. For activity relating to Projects instead of Projects (classic), use the projects_v2 event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Projects" repository or organization permission.
    ProjectCard,
    /// This event occurs when there is activity relating to a classic project. For more information, see "About projects (classic)." For information about the API to manage classic projects, see the GraphQL API documentation or "Projects (classic)" in the REST API documentation.
    ///
    /// For activity relating to a card or column on a project, use the project_card and project_column event. For activity relating to Projects instead of Projects (classic), use the projects_v2 event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Projects" repository or organization permission.
    Project,
    /// This event occurs when there is activity relating to a column on a classic project. For more information, see "About projects (classic)." For information about the API to manage classic projects, see the GraphQL API documentation or "Projects (classic)" in the REST API documentation.
    ///
    /// For activity relating to a project or a card on a project, use the project and project_card event. For activity relating to Projects instead of Projects (classic), use the projects_v2 event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Projects" repository or organization permission.
    ProjectColumn,
    /// This event occurs when there is activity relating to an organization-level project. For more information, see "About Projects." For information about the Projects API, see the GraphQL documentation.
    ///
    /// For activity relating to a item on a project, use the projects_v2_item event. For activity relating to Projects (classic), use the project, project_card, and project_column` events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Projects" organization permission.
    ///
    /// Note: Webhook events for projects are currently in beta and subject to change. To share feedback about projects webhooks with GitHub, see the Projects webhook feedback discussion.
    ProjectsV2,
    /// This event occurs when there is activity relating to an item on an organization-level project. For more information, see "About Projects." For information about the Projects API, see the GraphQL documentation.
    ///
    /// For activity relating to a project (instead of an item on a project), use the projects_v2 event. For activity relating to Projects (classic), use the project, project_card, and project_column events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Projects" organization permission.
    ///
    /// Note: Webhook events for projects are currently in beta and subject to change. To share feedback about projects webhooks with GitHub, see the Projects webhook feedback discussion.
    ProjectsV2Item,
    /// This event occurs when repository visibility changes from private to public. For more information, see "Setting repository visibility."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Metadata" repository permission.
    Public,
    /// This event occurs when there is activity on a pull request. For more information, see "About pull requests." For information about the APIs to manage pull requests, see the GraphQL API documentation or "Pulls" in the REST API documentation.
    ///
    /// For activity related to pull request reviews, pull request review comments, pull request comments, or pull request review threads, use the pull_request_review, pull_request_review_comment, issue_comment, or pull_request_review_thread events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull requests" repository permission.
    PullRequest,
    /// This event occurs when there is activity relating to a pull request review. A pull request review is a group of pull request review comments in addition to a body comment and a state. For more information, see "About pull request reviews." For information about the APIs to manage pull request reviews, see the GraphQL API documentation or "Pull request reviews" in the REST API documentation.
    ///
    /// For activity related to pull request review comments, pull request comments, or pull request review threads, use the pull_request_review_comment, issue_comment, or pull_request_review_thread events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull requests" repository permission.
    PullRequestReview,
    /// This event occurs when there is activity relating to a pull request review comment. A pull request review comment is a comment on a pull request's diff. For more information, see "Commenting on a pull request." For information about the APIs to manage pull request review comments, see the GraphQL API documentation or "Pull request review comments" in the REST API documentation.
    ///
    /// For activity related to pull request reviews, pull request comments, or pull request review threads, use the pull_request_review, issue_comment, or pull_request_review_thread events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull requests" repository permission.
    PullRequestReviewComment,
    /// This event occurs when there is activity relating to a comment thread on a pull request. For more information, see "About pull request reviews." For information about the APIs to manage pull request review comment threads, see the GraphQL API documentation or "Pull request reviews" in the REST API documentation.
    ///
    /// For activity related to pull request review comments, pull request comments, or pull request reviews, use the pull_request_review_comment, issue_comment, or pull_request_review events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull requests" repository permission.
    PullRequestReviewThread,
    /// This event occurs when a commit or tag is pushed.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    ///
    /// Note: An event will not be created when more than three tags are pushed at once.
    Push,
    /// This event occurs when there is activity relating to GitHub Packages. For more information, see "Introduction to GitHub Packages." For information about the APIs to manage GitHub Packages, see the GraphQL API documentation or "Packages" in the REST API documentation.
    ///
    /// To install this event on a GitHub App, the app must have at least read-level access for the "Packages" repository permission.
    ///
    /// Note: GitHub recommends that you use the newer package event instead.
    RegistryPackage,
    /// This event occurs when there is activity relating to releases. For more information, see "About releases." For information about the APIs to manage releases, see the GraphQL API documentation or "Releases" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    Release,
    /// This event occurs when there is activity relating to a repository security advisory. For more information about repository security advisories, see "About GitHub Security Advisories for repositories."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Repository security advisories" permission.
    RepositoryAdvisory,
    /// This event occurs when there is activity relating to repositories. For more information, see "About repositories." For information about the APIs to manage repositories, see the GraphQL documentation or "Repositories" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Metadata" repository permission.
    Repository,
    /// This event occurs when a GitHub App sends a POST request to /repos/{owner}/{repo}/dispatches. For more information, see the REST API documentation for creating a repository dispatch event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    RepositoryDispatch,
    /// This event occurs when a repository is imported to GitHub. For more information, see "Importing a repository with GitHub Importer." For more information about the API to manage imports, see the REST API documentation.
    RepositoryImport,
    /// This event occurs when there is activity relating to a security vulnerability alert in a repository.
    ///
    /// Note: This event is deprecated. Use the dependabot_alert event instead.
    RepositoryVulnerabilityAlert,
    /// This event occurs when there is activity relating to a secret scanning alert. For more information about secret scanning, see "About secret scanning." For information about the API to manage secret scanning alerts, see "Secret scanning" in the REST API documentation.
    ///
    /// For activity relating to secret scanning alert locations, use the secret_scanning_alert_location event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Secret scanning alerts" repository permission.
    SecretScanningAlert,
    /// This event occurs when there is activity relating to the locations of a secret in a secret scanning alert.
    ///
    /// For more information about secret scanning, see "About secret scanning." For information about the API to manage secret scanning alerts, see "Secret scanning" in the REST API documentation.
    ///
    /// For activity relating to secret scanning alerts, use the secret_scanning_alert event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Secret scanning alerts" repository permission.
    SecretScanningAlertLocation,
    /// This event occurs when there is activity relating to a security advisory that was reviewed by GitHub. A GitHub-reviewed security advisory provides information about security-related vulnerabilities in software on GitHub. For more information about security advisories, see "About GitHub Security Advisories for repositories." For information about the API to manage security advisories, see the GraphQL documentation.
    ///
    /// GitHub Dependabot alerts are also powered by the security advisory dataset. For more information, see "About Dependabot alerts."
    SecurityAdvisory,
    /// This event occurs when code security and analysis features are enabled or disabled for a repository. For more information, see "GitHub security features."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Administration" repository permission.
    SecurityAndAnalysis,
    /// This event occurs when there is activity relating to a sponsorship listing. For more information, see "About GitHub Sponsors." For information about the API to manage sponsors, see the GraphQL documentation.
    ///
    /// You can only create a sponsorship webhook on GitHub.com. For more information, see "Configuring webhooks for events in your sponsored account."
    Sponsorship,
    /// This event occurs when there is activity relating to repository stars. For more information about stars, see "Saving repositories with stars." For information about the APIs to manage stars, see the GraphQL documentation or "Starring" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Metadata" repository permission.
    Star,
    /// This event occurs when the status of a Git commit changes. For example, commits can be marked as error, failure, pending, or success. For more information, see "About status checks." For information about the APIs to manage commit statuses, see the GraphQL documentation or "Statuses" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Commit statuses" repository permission.
    Status,
    /// This event occurs when a team is added to a repository. For more information, see "Managing teams and people with access to your repository."
    ///
    /// For activity relating to teams, see the teams event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Members" organization permission.
    TeamAdd,
    /// This event occurs when there is activity relating to teams in an organization. For more information, see "About teams."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Members" organization permission.
    Team,
    /// This event occurs when there is activity relating to watching, or subscribing to, a repository. For more information about watching, see "Managing your subscriptions." For information about the APIs to manage watching, see "Watching" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Metadata" repository permission.
    Watch,
    /// This event occurs when a GitHub Actions workflow is manually triggered. For more information, see "Manually running a workflow."
    ///
    /// For activity relating to workflow runs, use the workflow_run event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Contents" repository permission.
    WorkflowDispatch,
    /// This event occurs when there is activity relating to a job in a GitHub Actions workflow. For more information, see "Using jobs in a workflow." For information about the API to manage workflow jobs, see "Workflow jobs" in the REST API documentation.
    ///
    /// For activity relating to a workflow run instead of a job in a workflow run, use the workflow_run event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Actions" repository permission.
    WorkflowJob,
    /// This event occurs when there is activity relating to a run of a GitHub Actions workflow. For more information, see "About workflows." For information about the APIs to manage workflow runs, see the GraphQL documentation or "Workflow runs" in the REST API documentation.
    ///
    /// For activity relating to a job in a workflow run, use the workflow_job event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Actions" repository permission.
    WorkflowRun,
    #[serde(untagged)]
    Unknown(String),
}

impl WebhookEventType {
    /// Parse (and verify) the payload for the specific event kind.
    pub fn parse_specific_payload(
        &self,
        data: serde_json::Value,
    ) -> Result<WebhookEventPayload, serde_json::Error> {
        match self {
            WebhookEventType::BranchProtectionRule => {
                Ok(WebhookEventPayload::BranchProtectionRuleWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::CheckRun => Ok(WebhookEventPayload::CheckRunWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::CheckSuite => Ok(WebhookEventPayload::CheckSuiteWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::CodeScanningAlert => {
                Ok(WebhookEventPayload::CodeScanningAlertWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::CommitComment => Ok(WebhookEventPayload::CommitCommentWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Create => Ok(WebhookEventPayload::CreateWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Delete => Ok(WebhookEventPayload::DeleteWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::DependabotAlert => {
                Ok(WebhookEventPayload::DependabotAlertWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::DeployKey => Ok(WebhookEventPayload::DeployKeyWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Deployment => Ok(WebhookEventPayload::DeploymentWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::DeploymentProtectionRule => {
                Ok(WebhookEventPayload::DeploymentProtectionRuleWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::DeploymentStatus => {
                Ok(WebhookEventPayload::DeploymentStatusWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Discussion => Ok(WebhookEventPayload::DiscussionWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::DiscussionComment => {
                Ok(WebhookEventPayload::DiscussionCommentWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Fork => Ok(WebhookEventPayload::ForkWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::GithubAppAuthorization => {
                Ok(WebhookEventPayload::GithubAppAuthorizationWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Gollum => Ok(WebhookEventPayload::GollumWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Installation => Ok(WebhookEventPayload::InstallationWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::InstallationRepositories => {
                Ok(WebhookEventPayload::InstallationRepositoriesWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::InstallationTarget => {
                Ok(WebhookEventPayload::InstallationTargetWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::IssueComment => Ok(WebhookEventPayload::IssueCommentWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Issues => Ok(WebhookEventPayload::IssuesWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Label => Ok(WebhookEventPayload::LabelWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::MarketplacePurchase => {
                Ok(WebhookEventPayload::MarketplacePurchaseWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Member => Ok(WebhookEventPayload::MemberWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Membership => Ok(WebhookEventPayload::MembershipWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::MergeGroup => Ok(WebhookEventPayload::MergeGroupWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Meta => Ok(WebhookEventPayload::MetaWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Milestone => Ok(WebhookEventPayload::MilestoneWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::OrgBlock => Ok(WebhookEventPayload::OrgBlockWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Organization => Ok(WebhookEventPayload::OrganizationWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Package => Ok(WebhookEventPayload::PackageWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::PageBuild => Ok(WebhookEventPayload::PageBuildWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::PersonalAccessTokenRequest => {
                Ok(WebhookEventPayload::PersonalAccessTokenRequestWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Ping => Ok(WebhookEventPayload::PingWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::ProjectCard => Ok(WebhookEventPayload::ProjectCardWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Project => Ok(WebhookEventPayload::ProjectWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::ProjectColumn => Ok(WebhookEventPayload::ProjectColumnWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::ProjectsV2 => Ok(WebhookEventPayload::ProjectsV2WebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::ProjectsV2Item => {
                Ok(WebhookEventPayload::ProjectsV2ItemWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Public => Ok(WebhookEventPayload::PublicWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::PullRequest => Ok(WebhookEventPayload::PullRequestWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::PullRequestReview => {
                Ok(WebhookEventPayload::PullRequestReviewWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::PullRequestReviewComment => {
                Ok(WebhookEventPayload::PullRequestReviewCommentWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::PullRequestReviewThread => {
                Ok(WebhookEventPayload::PullRequestReviewThreadWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Push => Ok(WebhookEventPayload::PushWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::RegistryPackage => {
                Ok(WebhookEventPayload::RegistryPackageWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Release => Ok(WebhookEventPayload::ReleaseWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::RepositoryAdvisory => {
                Ok(WebhookEventPayload::RepositoryAdvisoryWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Repository => Ok(WebhookEventPayload::RepositoryWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::RepositoryDispatch => {
                Ok(WebhookEventPayload::RepositoryDispatchWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::RepositoryImport => {
                Ok(WebhookEventPayload::RepositoryImportWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::RepositoryVulnerabilityAlert => Ok(
                WebhookEventPayload::RepositoryVulnerabilityAlertWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )),
            ),
            WebhookEventType::SecretScanningAlert => {
                Ok(WebhookEventPayload::SecretScanningAlertWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::SecretScanningAlertLocation => Ok(
                WebhookEventPayload::SecretScanningAlertLocationWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )),
            ),
            WebhookEventType::SecurityAdvisory => {
                Ok(WebhookEventPayload::SecurityAdvisoryWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::SecurityAndAnalysis => {
                Ok(WebhookEventPayload::SecurityAndAnalysisWebhookEvent(
                    Box::new(serde_json::from_value(data)?),
                ))
            }
            WebhookEventType::Sponsorship => Ok(WebhookEventPayload::SponsorshipWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Star => Ok(WebhookEventPayload::StarWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Status => Ok(WebhookEventPayload::StatusWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::TeamAdd => Ok(WebhookEventPayload::TeamAddWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Team => Ok(WebhookEventPayload::TeamWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Watch => Ok(WebhookEventPayload::WatchWebhookEvent(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::WorkflowDispatch => {
                Ok(WebhookEventPayload::WorkflowDispatchWebhookEvent(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::WorkflowJob => Ok(WebhookEventPayload::WorkflowJobWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::WorkflowRun => Ok(WebhookEventPayload::WorkflowRunWebhookEvent(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Unknown(_) => {
                Ok(WebhookEventPayload::UnknownWebhookEvent(Box::new(data)))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventInstallation {
    /// A full installation object which is present for `Installation*` related webhook events.
    Full(Box<Installation>),
    /// The minimal installation object is present for all other event types.
    Minimal(Box<EventInstallationId>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventInstallationId {
    pub id: InstallationId,
    pub node_id: String,
}

/// A repository in installation related webhook events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationEventRepository {
    pub id: RepositoryId,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
}

#[cfg(test)]
mod tests {
    use super::payload::*;
    use super::*;

    #[test]
    fn deserialize_installation_created() {
        let json = include_str!("../../tests/resources/installation_created_webhook_event.json");
        let event = WebhookEventType::Installation
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::InstallationWebhookEvent(install_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            install_event.action,
            InstallationWebhookEventAction::Created
        );
        assert_eq!(install_event.repositories.len(), 3);
        assert_eq!(install_event.repositories[2].name, "octocrab");
    }

    #[test]
    fn deserialize_installation_deleted() {
        let json = include_str!("../../tests/resources/installation_deleted_webhook_event.json");
        let event = WebhookEventType::Installation
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::InstallationWebhookEvent(install_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            install_event.action,
            InstallationWebhookEventAction::Deleted
        );
        assert_eq!(install_event.repositories.len(), 3);
        assert_eq!(install_event.repositories[2].name, "octocrab");
    }

    #[test]
    fn deserialize_installation_repositories_removed() {
        let json = include_str!(
            "../../tests/resources/installation_repositories_removed_webhook_event.json"
        );
        let event = WebhookEventType::InstallationRepositories
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::InstallationRepositoriesWebhookEvent(install_repositories_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            install_repositories_event.action,
            InstallationRepositoriesWebhookEventAction::Removed
        );
        assert_eq!(install_repositories_event.repositories_removed.len(), 1);
        assert_eq!(
            install_repositories_event.repository_selection,
            InstallationRepositoriesWebhookEventSelection::All
        );
    }

    #[test]
    fn deserialize_issue_comment_created() {
        let json = include_str!("../../tests/resources/issue_comment_created_webhook_event.json");
        let event = WebhookEventType::IssueComment
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::IssueCommentWebhookEvent(issue_comment_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            issue_comment_event.action,
            IssueCommentWebhookEventAction::Created
        );
        assert_eq!(*issue_comment_event.comment.id, 1633968123);
    }

    #[test]
    fn deserialize_issue_comment_deleted() {
        let json = include_str!("../../tests/resources/issue_comment_deleted_webhook_event.json");
        let event = WebhookEventType::IssueComment
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::IssueCommentWebhookEvent(issue_comment_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            issue_comment_event.action,
            IssueCommentWebhookEventAction::Deleted
        );
    }

    #[test]
    fn deserialize_issue_comment_edited() {
        let json = include_str!("../../tests/resources/issue_comment_edited_webhook_event.json");
        let event = WebhookEventType::IssueComment
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::IssueCommentWebhookEvent(issue_comment_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            issue_comment_event.action,
            IssueCommentWebhookEventAction::Edited
        );
        assert_eq!(issue_comment_event.changes.unwrap().body.from, "Old Body");
    }

    #[test]
    fn deserialize_issues_labeled() {
        let json = include_str!("../../tests/resources/issues_labeled_webhook_event.json");
        let event = WebhookEventType::Issues
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::IssuesWebhookEvent(issues_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(issues_event.action, IssuesWebhookEventAction::Labeled);
    }

    #[test]
    fn deserialize_issues_opened() {
        let json = include_str!("../../tests/resources/issues_opened_webhook_event.json");
        let event = WebhookEventType::Issues
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::IssuesWebhookEvent(issues_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(issues_event.action, IssuesWebhookEventAction::Opened);
    }

    #[test]
    fn deserialize_ping() {
        let json = include_str!("../../tests/resources/ping_webhook_event.json");
        let event = WebhookEventType::Ping
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::PingWebhookEvent(ping_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(ping_event.hook.unwrap().id, 423885699);
    }

    #[test]
    fn deserialize_pull_request_closed() {
        let json = include_str!("../../tests/resources/pull_request_closed_webhook_event.json");
        let event = WebhookEventType::PullRequest
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::PullRequestWebhookEvent(pull_request_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            pull_request_event.action,
            PullRequestWebhookEventAction::Closed
        );
        assert_eq!(pull_request_event.pull_request.number, 2);
    }

    #[test]
    fn deserialize_pull_request_opened() {
        let json = include_str!("../../tests/resources/pull_request_opened_webhook_event.json");
        let event = WebhookEventType::PullRequest
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::PullRequestWebhookEvent(pull_request_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            pull_request_event.action,
            PullRequestWebhookEventAction::Opened
        );
    }

    #[test]
    fn deserialize_pull_request_synchronize() {
        let json =
            include_str!("../../tests/resources/pull_request_synchronize_webhook_event.json");
        let event = WebhookEventType::PullRequest
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::PullRequestWebhookEvent(pull_request_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            pull_request_event.action,
            PullRequestWebhookEventAction::Synchronize
        );
    }

    #[test]
    fn deserialize_repository_deleted() {
        let json = include_str!("../../tests/resources/repository_deleted_webhook_event.json");
        let event = WebhookEventType::Repository
            .parse_specific_payload(serde_json::from_str(json).unwrap())
            .unwrap();
        let WebhookEventPayload::RepositoryWebhookEvent(repository_event) = event else {panic!(" event is of the wrong type {:?}", event)};
        assert_eq!(
            repository_event.action,
            RepositoryWebhookEventAction::Deleted
        );
    }
}
