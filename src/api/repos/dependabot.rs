//! Repository Dependabot API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot?apiVersion=2022-11-28)

use http::StatusCode;

use super::RepoRef;
use crate::models::dependabot::{
    DependabotAlert, DependabotAlertDismissedReason, DependabotAlertUpdateState,
};
use crate::models::repos::secrets::{
    CreateRepositorySecret, CreateRepositorySecretResponse, RepositorySecret,
};
use crate::models::PublicKey;
use crate::params::Direction;
use crate::{Octocrab, Page, Result};

/// Handler for repository Dependabot functionality.
pub struct RepoDependabotHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoDependabotHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Handler for managing repository Dependabot secrets.
    pub fn secrets(&self) -> RepoDependabotSecretsHandler<'octo> {
        RepoDependabotSecretsHandler::new(self.crab, self.repo.clone())
    }

    /// Handler for managing repository Dependabot alerts.
    pub fn alerts(&self) -> RepoDependabotAlertsHandler<'octo> {
        RepoDependabotAlertsHandler::new(self.crab, self.repo.clone())
    }

    /// Lists all Dependabot Alerts available in a repository.
    ///
    /// For advanced filtering and pagination, use [`RepoDependabotHandler::alerts`].
    pub async fn get_alerts(
        &self,
    ) -> Result<Page<crate::models::repos::dependabot::DependabotAlert>> {
        let route = format!("/{}/dependabot/alerts", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Lists single Dependabot Alert for a repository.
    ///
    /// For advanced alert operations, use [`RepoDependabotHandler::alerts`].
    pub async fn get_alert(
        &self,
        alert_number: u32,
    ) -> Result<crate::models::repos::dependabot::DependabotAlert> {
        let route = format!("/{}/dependabot/alerts/{}", self.repo, alert_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Updates a dependabot alert.
    ///
    /// For advanced alert operations, use [`RepoDependabotHandler::alerts`].
    pub async fn update_alert(
        &self,
        alert_number: u32,
        alert_update: Option<&crate::models::repos::dependabot::UpdateDependabotAlert<'_>>,
    ) -> Result<crate::models::repos::dependabot::DependabotAlert> {
        let route = format!("/{}/dependabot/alerts/{}", self.repo, alert_number);
        self.crab.patch(route, alert_update).await
    }
}

/// Handler for repository Dependabot secrets.
pub struct RepoDependabotSecretsHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoDependabotSecretsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Lists repository Dependabot secrets.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#list-repository-secrets)
    pub fn list(&self) -> ListRepoDependabotSecretsBuilder<'octo, '_> {
        ListRepoDependabotSecretsBuilder::new(self)
    }

    /// Gets the repository public key for Dependabot secrets encryption.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#get-a-repository-public-key)
    pub async fn get_public_key(&self) -> Result<PublicKey> {
        let route = format!("/{}/dependabot/secrets/public-key", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Gets a repository Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#get-a-repository-secret)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<RepositorySecret> {
        let route = format!("/{}/dependabot/secrets/{}", self.repo, secret_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates or updates a repository Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#create-or-update-a-repository-secret)
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateRepositorySecret<'_>,
    ) -> Result<CreateRepositorySecretResponse> {
        let route = format!("/{}/dependabot/secrets/{}", self.repo, secret_name.as_ref());
        let resp = self.crab._put(route, Some(secret)).await?;
        let resp = crate::map_github_error(resp).await?;
        match resp.status() {
            StatusCode::CREATED => Ok(CreateRepositorySecretResponse::Created),
            StatusCode::NO_CONTENT => Ok(CreateRepositorySecretResponse::Updated),
            status => Err(crate::Error::Other {
                source: format!("Unexpected status code from request: {}", status.as_str()).into(),
                backtrace: snafu::Backtrace::capture(),
            }),
        }
    }

    /// Deletes a repository Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#delete-a-repository-secret)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!("/{}/dependabot/secrets/{}", self.repo, secret_name.as_ref());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// Builder for listing repository Dependabot secrets.
#[derive(serde::Serialize)]
pub struct ListRepoDependabotSecretsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoDependabotSecretsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRepoDependabotSecretsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoDependabotSecretsHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the request and returns the resulting page of secrets.
    pub async fn send(self) -> Result<Page<RepositorySecret>> {
        let route = format!("/{}/dependabot/secrets", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for repository Dependabot alerts.
pub struct RepoDependabotAlertsHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoDependabotAlertsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Lists repository Dependabot alerts.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/alerts?apiVersion=2022-11-28#list-dependabot-alerts-for-a-repository)
    pub fn list(&self) -> ListRepoDependabotAlertsBuilder<'octo, '_> {
        ListRepoDependabotAlertsBuilder::new(self)
    }

    /// Gets a repository Dependabot alert.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/alerts?apiVersion=2022-11-28#get-a-dependabot-alert)
    pub async fn get(&self, alert_number: u64) -> Result<DependabotAlert> {
        let route = format!("/{}/dependabot/alerts/{}", self.repo, alert_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Updates a repository Dependabot alert.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/alerts?apiVersion=2022-11-28#update-a-dependabot-alert)
    pub fn update(&self, alert_number: u64) -> UpdateRepoDependabotAlertBuilder<'octo, '_> {
        UpdateRepoDependabotAlertBuilder::new(self, alert_number)
    }
}

/// Builder for listing repository Dependabot alerts.
#[derive(serde::Serialize)]
pub struct ListRepoDependabotAlertsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoDependabotAlertsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecosystem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epss_percentage: Option<String>,
}

impl<'octo, 'r> ListRepoDependabotAlertsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoDependabotAlertsHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            severity: None,
            ecosystem: None,
            package: None,
            manifest: None,
            scope: None,
            sort: None,
            direction: None,
            page: None,
            per_page: None,
            before: None,
            after: None,
            first: None,
            last: None,
            epss_percentage: None,
        }
    }

    /// Filter by state (e.g. `auto_dismissed`, `dismissed`, `fixed`, `open`).
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Filter by severity (e.g. `low`, `medium`, `high`, `critical`).
    pub fn severity(mut self, severity: impl Into<String>) -> Self {
        self.severity = Some(severity.into());
        self
    }

    /// Filter by ecosystem (e.g. `npm`, `pip`, `cargo`, `maven`, etc.).
    pub fn ecosystem(mut self, ecosystem: impl Into<String>) -> Self {
        self.ecosystem = Some(ecosystem.into());
        self
    }

    /// Filter by package name.
    pub fn package(mut self, package: impl Into<String>) -> Self {
        self.package = Some(package.into());
        self
    }

    /// Filter by manifest path.
    pub fn manifest(mut self, manifest: impl Into<String>) -> Self {
        self.manifest = Some(manifest.into());
        self
    }

    /// Filter by scope (e.g. `development`, `runtime`).
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Property by which to sort the results (e.g. `created`, `updated`).
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Direction to sort results.
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Cursor for pagination (before).
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Cursor for pagination (after).
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Number of results to return from the beginning.
    pub fn first(mut self, first: impl Into<u32>) -> Self {
        self.first = Some(first.into());
        self
    }

    /// Number of results to return from the end.
    pub fn last(mut self, last: impl Into<u32>) -> Self {
        self.last = Some(last.into());
        self
    }

    /// Filter by EPSS percentage.
    pub fn epss_percentage(mut self, epss_percentage: impl Into<String>) -> Self {
        self.epss_percentage = Some(epss_percentage.into());
        self
    }

    /// Sends the request and returns the resulting page of alerts.
    pub async fn send(self) -> Result<Page<DependabotAlert>> {
        let route = format!("/{}/dependabot/alerts", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for updating a repository Dependabot alert.
#[derive(serde::Serialize)]
pub struct UpdateRepoDependabotAlertBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoDependabotAlertsHandler<'octo>,
    #[serde(skip)]
    alert_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<DependabotAlertUpdateState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissed_reason: Option<DependabotAlertDismissedReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissed_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
}

impl<'octo, 'r> UpdateRepoDependabotAlertBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoDependabotAlertsHandler<'octo>, alert_number: u64) -> Self {
        Self {
            handler,
            alert_number,
            state: None,
            dismissed_reason: None,
            dismissed_comment: None,
            assignees: None,
        }
    }

    /// The state to set (`dismissed` or `open`).
    pub fn state(mut self, state: impl Into<DependabotAlertUpdateState>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// A reason for dismissing the alert.
    pub fn dismissed_reason(mut self, reason: impl Into<DependabotAlertDismissedReason>) -> Self {
        self.dismissed_reason = Some(reason.into());
        self
    }

    /// An optional comment associated with dismissing the alert.
    pub fn dismissed_comment(mut self, comment: impl Into<String>) -> Self {
        self.dismissed_comment = Some(comment.into());
        self
    }

    /// Usernames to assign to this Dependabot alert.
    pub fn assignees(mut self, assignees: impl Into<Vec<String>>) -> Self {
        self.assignees = Some(assignees.into());
        self
    }

    /// Sends the update request.
    pub async fn send(self) -> Result<DependabotAlert> {
        let route = format!(
            "/{}/dependabot/alerts/{}",
            self.handler.repo, self.alert_number
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}
