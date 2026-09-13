//! Organization Dependabot API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot?apiVersion=2022-11-28)

use http::StatusCode;

use crate::models::dependabot::{
    DependabotAlert, DependabotDefaultRepositoryAccessLevel, DependabotRepositoryAccess,
    SelectedRepositories, SetDependabotDefaultRepositoryAccessLevel, SetSelectedRepositories,
    UpdateDependabotRepositoryAccess,
};
use crate::models::orgs::secrets::{
    CreateOrganizationSecret, CreateOrganizationSecretResponse, OrganizationSecret,
};
use crate::models::{PublicKey, RepositoryId};
use crate::params::Direction;
use crate::{Octocrab, Page, Result};

/// Handler for organization Dependabot functionality.
pub struct OrgDependabotHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgDependabotHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Handler for managing organization Dependabot secrets.
    pub fn secrets(&self) -> OrgDependabotSecretsHandler<'octo> {
        OrgDependabotSecretsHandler::new(self.crab, self.owner.clone())
    }

    /// Handler for managing organization Dependabot alerts.
    pub fn alerts(&self) -> OrgDependabotAlertsHandler<'octo> {
        OrgDependabotAlertsHandler::new(self.crab, self.owner.clone())
    }

    /// Handler for managing organization Dependabot repository access.
    pub fn repository_access(&self) -> OrgDependabotRepositoryAccessHandler<'octo> {
        OrgDependabotRepositoryAccessHandler::new(self.crab, self.owner.clone())
    }
}

/// Handler for organization Dependabot secrets.
pub struct OrgDependabotSecretsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgDependabotSecretsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Lists organization Dependabot secrets.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#list-organization-secrets)
    pub fn list(&self) -> ListOrgDependabotSecretsBuilder<'octo, '_> {
        ListOrgDependabotSecretsBuilder::new(self)
    }

    /// Gets the organization public key for Dependabot secrets encryption.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#get-an-organization-public-key)
    pub async fn get_public_key(&self) -> Result<PublicKey> {
        let route = format!("/orgs/{}/dependabot/secrets/public-key", self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// Gets an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#get-an-organization-secret)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<OrganizationSecret> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}",
            self.owner,
            secret_name.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Creates or updates an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#create-or-update-an-organization-secret)
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateOrganizationSecret<'_>,
    ) -> Result<CreateOrganizationSecretResponse> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}",
            self.owner,
            secret_name.as_ref()
        );
        let resp = self.crab._put(route, Some(secret)).await?;
        let resp = crate::map_github_error(resp).await?;
        match resp.status() {
            StatusCode::CREATED => Ok(CreateOrganizationSecretResponse::Created),
            StatusCode::NO_CONTENT => Ok(CreateOrganizationSecretResponse::Updated),
            status => Err(crate::Error::Other {
                source: format!("Unexpected status code from request: {}", status.as_str()).into(),
                backtrace: snafu::Backtrace::capture(),
            }),
        }
    }

    /// Deletes an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#delete-an-organization-secret)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}",
            self.owner,
            secret_name.as_ref()
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Handler for managing selected repositories for an organization Dependabot secret.
    pub fn repositories(
        &self,
        secret_name: impl Into<String>,
    ) -> DependabotSecretRepositoriesHandler<'octo> {
        DependabotSecretRepositoriesHandler::new(self.crab, self.owner.clone(), secret_name.into())
    }
}

/// Builder for listing organization Dependabot secrets.
#[derive(serde::Serialize)]
pub struct ListOrgDependabotSecretsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgDependabotSecretsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgDependabotSecretsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgDependabotSecretsHandler<'octo>) -> Self {
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
    pub async fn send(self) -> Result<Page<OrganizationSecret>> {
        let route = format!("/orgs/{}/dependabot/secrets", self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for selected repositories for an organization Dependabot secret.
pub struct DependabotSecretRepositoriesHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    secret_name: String,
}

impl<'octo> DependabotSecretRepositoriesHandler<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        owner: impl Into<String>,
        secret_name: impl Into<String>,
    ) -> Self {
        Self {
            crab,
            owner: owner.into(),
            secret_name: secret_name.into(),
        }
    }

    /// Lists selected repositories for an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#list-selected-repositories-for-an-organization-secret)
    pub async fn list(&self) -> Result<SelectedRepositories> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}/repositories",
            self.owner, self.secret_name
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets selected repositories for an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#set-selected-repositories-for-an-organization-secret)
    pub async fn set(&self, repository_ids: &[RepositoryId]) -> Result<()> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}/repositories",
            self.owner, self.secret_name
        );
        let body = SetSelectedRepositories {
            selected_repository_ids: repository_ids,
        };
        let resp = self.crab._put(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Adds a selected repository to an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#add-selected-repository-to-an-organization-secret)
    pub async fn add(&self, repository_id: RepositoryId) -> Result<()> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}/repositories/{}",
            self.owner, self.secret_name, repository_id
        );
        let resp = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Removes a selected repository from an organization Dependabot secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/secrets?apiVersion=2022-11-28#remove-selected-repository-from-an-organization-secret)
    pub async fn remove(&self, repository_id: RepositoryId) -> Result<()> {
        let route = format!(
            "/orgs/{}/dependabot/secrets/{}/repositories/{}",
            self.owner, self.secret_name, repository_id
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// Handler for organization Dependabot alerts.
pub struct OrgDependabotAlertsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgDependabotAlertsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Lists organization Dependabot alerts.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/alerts?apiVersion=2022-11-28#list-dependabot-alerts-for-an-organization)
    pub fn list(&self) -> ListOrgDependabotAlertsBuilder<'octo, '_> {
        ListOrgDependabotAlertsBuilder::new(self)
    }
}

/// Builder for listing organization Dependabot alerts.
#[derive(serde::Serialize)]
pub struct ListOrgDependabotAlertsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgDependabotAlertsHandler<'octo>,
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

impl<'octo, 'r> ListOrgDependabotAlertsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgDependabotAlertsHandler<'octo>) -> Self {
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
        let route = format!("/orgs/{}/dependabot/alerts", self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for organization Dependabot repository access.
pub struct OrgDependabotRepositoryAccessHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgDependabotRepositoryAccessHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Lists repositories that Dependabot can access for the organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/repository-access?apiVersion=2022-11-28#list-repositories-dependabot-can-access)
    pub fn list(&self) -> ListOrgDependabotRepositoryAccessBuilder<'octo, '_> {
        ListOrgDependabotRepositoryAccessBuilder::new(self)
    }

    /// Updates the repositories that Dependabot can access for the organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/repository-access?apiVersion=2022-11-28#update-the-dependabot-repository-access-list)
    pub fn update(&self) -> UpdateOrgDependabotRepositoryAccessBuilder<'octo, '_> {
        UpdateOrgDependabotRepositoryAccessBuilder::new(self)
    }

    /// Sets the default repository access level for Dependabot in the organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependabot/repository-access?apiVersion=2022-11-28#set-the-default-repository-access-level)
    pub async fn set_default_level(
        &self,
        default_level: DependabotDefaultRepositoryAccessLevel,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/dependabot/repository-access/default-level",
            self.owner
        );
        let body = SetDependabotDefaultRepositoryAccessLevel { default_level };
        let resp = self.crab._put(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// Builder for listing repositories Dependabot can access.
#[derive(serde::Serialize)]
pub struct ListOrgDependabotRepositoryAccessBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgDependabotRepositoryAccessHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
}

impl<'octo, 'r> ListOrgDependabotRepositoryAccessBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgDependabotRepositoryAccessHandler<'octo>) -> Self {
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
            "/orgs/{}/dependabot/repository-access",
            self.handler.owner
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for updating the repositories Dependabot can access.
pub struct UpdateOrgDependabotRepositoryAccessBuilder<'octo, 'r> {
    handler: &'r OrgDependabotRepositoryAccessHandler<'octo>,
    repository_ids_to_add: Vec<RepositoryId>,
    repository_ids_to_remove: Vec<RepositoryId>,
}

impl<'octo, 'r> UpdateOrgDependabotRepositoryAccessBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgDependabotRepositoryAccessHandler<'octo>) -> Self {
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
            "/orgs/{}/dependabot/repository-access",
            self.handler.owner
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
