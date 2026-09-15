//! The Enterprise API.

use crate::api::repos::secret_scanning_alerts::Params;
use crate::models::dependabot::{
    DependabotAlert, DependabotDefaultRepositoryAccessLevel, DependabotRepositoryAccess,
    SetDependabotDefaultRepositoryAccessLevel, UpdateDependabotRepositoryAccess,
};
use crate::models::RepositoryId;
use crate::params::Direction;
use crate::{Octocrab, Page, Result};

/// A client to GitHub's enterprise API.
///
/// Created with [`Octocrab::enterprises`].
pub struct EnterpriseHandler<'octo> {
    crab: &'octo Octocrab,
    enterprise: String,
}

impl<'octo> EnterpriseHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, enterprise: String) -> Self {
        Self { crab, enterprise }
    }

    /// Handle secret scanning alerts for the enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/secret-scanning?apiVersion=2022-11-28#list-secret-scanning-alerts-for-an-enterprise)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .enterprises("my-enterprise")
    ///     .secret_scanning()
    ///     .get_alerts()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn secret_scanning(&self) -> EnterpriseSecretScanningAlertsHandler<'_> {
        EnterpriseSecretScanningAlertsHandler::new(self)
    }

    /// Handle secret scanning alerts for the enterprise (alias for [`secret_scanning`][EnterpriseHandler::secret_scanning]).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .enterprises("my-enterprise")
    ///     .secrets_scanning()
    ///     .get_alerts()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn secrets_scanning(&self) -> EnterpriseSecretScanningAlertsHandler<'_> {
        self.secret_scanning()
    }

    /// Handle Dependabot for the enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/dependabot?apiVersion=2022-11-28)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .alerts()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn dependabot(&self) -> EnterpriseDependabotHandler<'octo> {
        EnterpriseDependabotHandler::new(self.crab, self.enterprise.clone())
    }
}

/// A client to GitHub's enterprise Secret Scanning API.
///
/// Created with [`EnterpriseHandler::secret_scanning`].
pub struct EnterpriseSecretScanningAlertsHandler<'octo> {
    handler: &'octo EnterpriseHandler<'octo>,
    params: Params,
}

impl<'octo> EnterpriseSecretScanningAlertsHandler<'octo> {
    pub(crate) fn new(enterprise: &'octo EnterpriseHandler<'octo>) -> Self {
        Self {
            handler: enterprise,
            params: Params::default(),
        }
    }

    /// Lists secret scanning alerts for eligible repositories in an enterprise.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab.enterprises("my-enterprise")
    ///     .secret_scanning()
    ///     .direction("asc")
    ///     .get_alerts()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_alerts(
        &self,
    ) -> crate::Result<crate::Page<crate::models::repos::secret_scanning_alert::SecretScanningAlert>>
    {
        let route = format!(
            "/enterprises/{}/secret-scanning/alerts",
            self.handler.enterprise
        );
        self.handler.crab.get(route, Some(&self.params)).await
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.params.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.params.page = Some(page.into());
        self
    }

    /// Filter Secret Scanning Alerts by state.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.params.state = Some(state.into());
        self
    }

    /// Filter Secret Scanning Alerts by resolution.
    pub fn resolution(mut self, resolution: impl Into<Vec<String>>) -> Self {
        self.params.resolution = Some(resolution.into());
        self
    }

    /// Filter Secret Scanning Alerts by validity.
    pub fn validity(mut self, validity: impl Into<Vec<String>>) -> Self {
        self.params.validity = Some(validity.into());
        self
    }

    /// Filter Secret Scanning Alerts by secret_type.
    pub fn secret_type(mut self, secret_type: impl Into<String>) -> Self {
        self.params.secret_type = Some(secret_type.into());
        self
    }

    /// Filter Secret Scanning Alerts by multi repo alerts.
    pub fn is_multi_repo(mut self, is_multi_repo: impl Into<bool>) -> Self {
        self.params.is_multi_repo = Some(is_multi_repo.into());
        self
    }

    /// Filter Secret Scanning Alerts by publicly leaked.
    pub fn is_publicly_leaked(mut self, is_publicly_leaked: impl Into<bool>) -> Self {
        self.params.is_publicly_leaked = Some(is_publicly_leaked.into());
        self
    }

    /// Hide secrets in results.
    pub fn hide_secret(mut self, hide_secret: impl Into<bool>) -> Self {
        self.params.hide_secret = Some(hide_secret.into());
        self
    }

    /// Filter Secret Scanning Alerts by push protection bypass status.
    pub fn is_bypassed(mut self, is_bypassed: impl Into<bool>) -> Self {
        self.params.is_bypassed = Some(is_bypassed.into());
        self
    }

    /// Filter Secret Scanning Alerts by after cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.params.after = Some(after.into());
        self
    }

    /// Filter Secret Scanning Alerts by before cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.params.before = Some(before.into());
        self
    }

    /// Sort Secret Scanning Alerts.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.params.sort = Some(sort.into());
        self
    }

    /// Sort direction of Secret Scanning Alerts.
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.params.direction = Some(direction.into());
        self
    }
}

/// Handler for enterprise Dependabot functionality.
pub struct EnterpriseDependabotHandler<'octo> {
    crab: &'octo Octocrab,
    enterprise: String,
}

impl<'octo> EnterpriseDependabotHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, enterprise: impl Into<String>) -> Self {
        Self {
            crab,
            enterprise: enterprise.into(),
        }
    }

    /// Handler for managing enterprise Dependabot alerts.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .alerts()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn alerts(&self) -> EnterpriseDependabotAlertsHandler<'octo> {
        EnterpriseDependabotAlertsHandler::new(self.crab, self.enterprise.clone())
    }

    /// Handler for managing enterprise Dependabot repository access.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let access = octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .repository_access()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn repository_access(&self) -> EnterpriseDependabotRepositoryAccessHandler<'octo> {
        EnterpriseDependabotRepositoryAccessHandler::new(self.crab, self.enterprise.clone())
    }
}

/// Handler for enterprise Dependabot alerts.
pub struct EnterpriseDependabotAlertsHandler<'octo> {
    crab: &'octo Octocrab,
    enterprise: String,
}

impl<'octo> EnterpriseDependabotAlertsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, enterprise: impl Into<String>) -> Self {
        Self {
            crab,
            enterprise: enterprise.into(),
        }
    }

    /// Lists enterprise Dependabot alerts.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/alerts?apiVersion=2022-11-28#list-dependabot-alerts-for-an-enterprise)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .alerts()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListEnterpriseDependabotAlertsBuilder<'octo, '_> {
        ListEnterpriseDependabotAlertsBuilder::new(self)
    }
}

/// Builder for listing enterprise Dependabot alerts.
#[derive(serde::Serialize)]
pub struct ListEnterpriseDependabotAlertsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r EnterpriseDependabotAlertsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecosystem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<String>,
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

impl<'octo, 'r> ListEnterpriseDependabotAlertsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r EnterpriseDependabotAlertsHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            severity: None,
            ecosystem: None,
            package: None,
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
        let route = format!("/enterprises/{}/dependabot/alerts", self.handler.enterprise);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for enterprise Dependabot repository access.
pub struct EnterpriseDependabotRepositoryAccessHandler<'octo> {
    crab: &'octo Octocrab,
    enterprise: String,
}

impl<'octo> EnterpriseDependabotRepositoryAccessHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, enterprise: impl Into<String>) -> Self {
        Self {
            crab,
            enterprise: enterprise.into(),
        }
    }

    /// Lists repositories that Dependabot can access for the enterprise.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/repository-access?apiVersion=2022-11-28#list-repositories-dependabot-can-access)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let access = octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .repository_access()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListEnterpriseDependabotRepositoryAccessBuilder<'octo, '_> {
        ListEnterpriseDependabotRepositoryAccessBuilder::new(self)
    }

    /// Updates the repositories that Dependabot can access for the enterprise.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/repository-access?apiVersion=2022-11-28#update-the-dependabot-repository-access-list)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .repository_access()
    ///     .update()
    ///     .add_repository(12345u64)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self) -> UpdateEnterpriseDependabotRepositoryAccessBuilder<'octo, '_> {
        UpdateEnterpriseDependabotRepositoryAccessBuilder::new(self)
    }

    /// Sets the default repository access level for Dependabot in the enterprise.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/repository-access?apiVersion=2022-11-28#set-the-default-repository-access-level)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::dependabot::DependabotDefaultRepositoryAccessLevel;
    ///
    /// octocrab
    ///     .enterprises("my-enterprise")
    ///     .dependabot()
    ///     .repository_access()
    ///     .set_default_level(DependabotDefaultRepositoryAccessLevel::Internal)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_default_level(
        &self,
        default_level: DependabotDefaultRepositoryAccessLevel,
    ) -> Result<()> {
        let route = format!(
            "/enterprises/{}/dependabot/repository-access/default-level",
            self.enterprise
        );
        let body = SetDependabotDefaultRepositoryAccessLevel { default_level };
        let resp = self.crab._put(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// Builder for listing repositories Dependabot can access in an enterprise.
#[derive(serde::Serialize)]
pub struct ListEnterpriseDependabotRepositoryAccessBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r EnterpriseDependabotRepositoryAccessHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
}

impl<'octo, 'r> ListEnterpriseDependabotRepositoryAccessBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r EnterpriseDependabotRepositoryAccessHandler<'octo>) -> Self {
        Self {
            handler,
            page: None,
            per_page: None,
        }
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

    /// Sends the request and returns the repository access configuration.
    pub async fn send(self) -> Result<DependabotRepositoryAccess> {
        let route = format!(
            "/enterprises/{}/dependabot/repository-access",
            self.handler.enterprise
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for updating the repositories Dependabot can access in an enterprise.
pub struct UpdateEnterpriseDependabotRepositoryAccessBuilder<'octo, 'r> {
    handler: &'r EnterpriseDependabotRepositoryAccessHandler<'octo>,
    repository_ids_to_add: Vec<RepositoryId>,
    repository_ids_to_remove: Vec<RepositoryId>,
}

impl<'octo, 'r> UpdateEnterpriseDependabotRepositoryAccessBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r EnterpriseDependabotRepositoryAccessHandler<'octo>) -> Self {
        Self {
            handler,
            repository_ids_to_add: Vec::new(),
            repository_ids_to_remove: Vec::new(),
        }
    }

    /// Add a repository ID to the access list.
    pub fn add_repository(mut self, id: impl Into<RepositoryId>) -> Self {
        self.repository_ids_to_add.push(id.into());
        self
    }

    /// Add multiple repository IDs to the access list.
    pub fn add_repositories(mut self, ids: impl IntoIterator<Item = RepositoryId>) -> Self {
        self.repository_ids_to_add.extend(ids);
        self
    }

    /// Remove a repository ID from the access list.
    pub fn remove_repository(mut self, id: impl Into<RepositoryId>) -> Self {
        self.repository_ids_to_remove.push(id.into());
        self
    }

    /// Remove multiple repository IDs from the access list.
    pub fn remove_repositories(mut self, ids: impl IntoIterator<Item = RepositoryId>) -> Self {
        self.repository_ids_to_remove.extend(ids);
        self
    }

    /// Sends the update request.
    pub async fn send(self) -> Result<()> {
        let route = format!(
            "/enterprises/{}/dependabot/repository-access",
            self.handler.enterprise
        );
        let body = UpdateDependabotRepositoryAccess {
            repository_ids_to_add: if self.repository_ids_to_add.is_empty() {
                None
            } else {
                Some(&self.repository_ids_to_add)
            },
            repository_ids_to_remove: if self.repository_ids_to_remove.is_empty() {
                None
            } else {
                Some(&self.repository_ids_to_remove)
            },
        };
        let resp = self.handler.crab._patch(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}
