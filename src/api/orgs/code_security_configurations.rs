use super::OrgHandler;
use crate::models::code_security::{
    CodeSecurityConfiguration, CodeSecurityConfigurationRepositories,
    CodeSecurityDefaultConfiguration,
};
use crate::{Page, Result};

/// A client to GitHub's organization Code Security Configurations API.
///
/// Created with [`OrgHandler::code_security`].
pub struct OrgCodeSecurityHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgCodeSecurityHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Get code security configurations for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-code-security-configurations-for-an-organization)
    pub fn list<'c>(&'c self) -> ListOrgCodeSecurityConfigsBuilder<'octo, 'b, 'c> {
        ListOrgCodeSecurityConfigsBuilder::new(self)
    }

    /// Create a code security configuration for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#create-a-code-security-configuration)
    pub async fn create(
        &self,
        config: &impl serde::Serialize,
    ) -> Result<CodeSecurityConfiguration> {
        let route = format!("/orgs/{}/code-security/configurations", self.handler.owner);
        self.handler.crab.post(route, Some(config)).await
    }

    /// Get default code security configurations for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-default-code-security-configurations)
    pub async fn get_defaults(&self) -> Result<Vec<CodeSecurityDefaultConfiguration>> {
        let route = format!(
            "/orgs/{}/code-security/configurations/defaults",
            self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Detach configurations from repositories for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#detach-configurations-from-repositories)
    pub async fn detach(&self, body: &impl serde::Serialize) -> Result<()> {
        let route = format!(
            "/orgs/{}/code-security/configurations/detach",
            self.handler.owner
        );
        let response = self.handler.crab._delete(route, Some(body)).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Get a code security configuration for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-a-code-security-configuration)
    pub async fn get(&self, configuration_id: u64) -> Result<CodeSecurityConfiguration> {
        let route = format!(
            "/orgs/{}/code-security/configurations/{}",
            self.handler.owner, configuration_id
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a code security configuration for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#update-a-code-security-configuration)
    pub async fn update(
        &self,
        configuration_id: u64,
        config: &impl serde::Serialize,
    ) -> Result<CodeSecurityConfiguration> {
        let route = format!(
            "/orgs/{}/code-security/configurations/{}",
            self.handler.owner, configuration_id
        );
        self.handler.crab.patch(route, Some(config)).await
    }

    /// Delete a code security configuration for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#delete-a-code-security-configuration)
    pub async fn delete(&self, configuration_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/code-security/configurations/{}",
            self.handler.owner, configuration_id
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Attach a configuration to repositories for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#attach-a-configuration-to-repositories)
    pub async fn attach(&self, configuration_id: u64, body: &impl serde::Serialize) -> Result<()> {
        let route = format!(
            "/orgs/{}/code-security/configurations/{}/attach",
            self.handler.owner, configuration_id
        );
        let response = self.handler.crab._post(route, Some(body)).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Set a code security configuration as a default for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#set-a-code-security-configuration-as-a-default-for-an-organization)
    pub async fn set_defaults(
        &self,
        configuration_id: u64,
        body: &impl serde::Serialize,
    ) -> Result<CodeSecurityDefaultConfiguration> {
        let route = format!(
            "/orgs/{}/code-security/configurations/{}/defaults",
            self.handler.owner, configuration_id
        );
        self.handler.crab.put(route, Some(body)).await
    }

    /// Get repositories associated with a code security configuration for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-repositories-associated-with-a-code-security-configuration)
    pub fn repositories<'c>(
        &'c self,
        configuration_id: u64,
    ) -> ListOrgCodeSecurityConfigRepositoriesBuilder<'octo, 'b, 'c> {
        ListOrgCodeSecurityConfigRepositoriesBuilder::new(self, configuration_id)
    }
}

/// Builder for listing organization code security configurations.
#[derive(serde::Serialize)]
pub struct ListOrgCodeSecurityConfigsBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c OrgCodeSecurityHandler<'octo, 'b>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
}

impl<'octo, 'b, 'c> ListOrgCodeSecurityConfigsBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c OrgCodeSecurityHandler<'octo, 'b>) -> Self {
        Self {
            handler,
            before: None,
            after: None,
            per_page: None,
        }
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub async fn send(self) -> Result<Page<CodeSecurityConfiguration>> {
        let route = format!(
            "/orgs/{}/code-security/configurations",
            self.handler.handler.owner
        );
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing repositories associated with a code security configuration.
#[derive(serde::Serialize)]
pub struct ListOrgCodeSecurityConfigRepositoriesBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c OrgCodeSecurityHandler<'octo, 'b>,
    #[serde(skip)]
    configuration_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
}

impl<'octo, 'b, 'c> ListOrgCodeSecurityConfigRepositoriesBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(
        handler: &'c OrgCodeSecurityHandler<'octo, 'b>,
        configuration_id: u64,
    ) -> Self {
        Self {
            handler,
            configuration_id,
            before: None,
            after: None,
            per_page: None,
            status: None,
        }
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub async fn send(self) -> Result<Page<CodeSecurityConfigurationRepositories>> {
        let route = format!(
            "/orgs/{}/code-security/configurations/{}/repositories",
            self.handler.handler.owner, self.configuration_id
        );
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
