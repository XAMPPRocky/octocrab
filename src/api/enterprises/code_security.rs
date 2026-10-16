use super::EnterpriseHandler;
use crate::models::code_security::{
    CodeSecurityConfiguration, CodeSecurityConfigurationRepositories,
    CodeSecurityDefaultConfiguration,
};
use crate::{Page, Result};

/// A client to GitHub's enterprise Code Security Configurations API.
///
/// Created with [`EnterpriseHandler::code_security`].
pub struct EnterpriseCodeSecurityHandler<'octo, 'b> {
    handler: &'b EnterpriseHandler<'octo>,
}

impl<'octo, 'b> EnterpriseCodeSecurityHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b EnterpriseHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Get code security configurations for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-code-security-configurations-for-an-enterprise)
    pub fn list<'c>(&'c self) -> ListEnterpriseCodeSecurityConfigsBuilder<'octo, 'b, 'c> {
        ListEnterpriseCodeSecurityConfigsBuilder::new(self)
    }

    /// Create a code security configuration for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#create-a-code-security-configuration-for-an-enterprise)
    pub async fn create(
        &self,
        config: &impl serde::Serialize,
    ) -> Result<CodeSecurityConfiguration> {
        let route = format!(
            "/enterprises/{}/code-security/configurations",
            self.handler.enterprise
        );
        self.handler.crab.post(route, Some(config)).await
    }

    /// Get default code security configurations for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-default-code-security-configurations-for-an-enterprise)
    pub async fn get_defaults(&self) -> Result<Vec<CodeSecurityDefaultConfiguration>> {
        let route = format!(
            "/enterprises/{}/code-security/configurations/defaults",
            self.handler.enterprise
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Retrieve a code security configuration of an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#retrieve-a-code-security-configuration-of-an-enterprise)
    pub async fn get(&self, configuration_id: u64) -> Result<CodeSecurityConfiguration> {
        let route = format!(
            "/enterprises/{}/code-security/configurations/{}",
            self.handler.enterprise, configuration_id
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a custom code security configuration for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#update-a-custom-code-security-configuration-for-an-enterprise)
    pub async fn update(
        &self,
        configuration_id: u64,
        config: &impl serde::Serialize,
    ) -> Result<CodeSecurityConfiguration> {
        let route = format!(
            "/enterprises/{}/code-security/configurations/{}",
            self.handler.enterprise, configuration_id
        );
        self.handler.crab.patch(route, Some(config)).await
    }

    /// Delete a code security configuration for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#delete-a-code-security-configuration-for-an-enterprise)
    pub async fn delete(&self, configuration_id: u64) -> Result<()> {
        let route = format!(
            "/enterprises/{}/code-security/configurations/{}",
            self.handler.enterprise, configuration_id
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Attach an enterprise configuration to repositories.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#attach-an-enterprise-configuration-to-repositories)
    pub async fn attach(&self, configuration_id: u64, body: &impl serde::Serialize) -> Result<()> {
        let route = format!(
            "/enterprises/{}/code-security/configurations/{}/attach",
            self.handler.enterprise, configuration_id
        );
        let response = self.handler.crab._post(route, Some(body)).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Set a code security configuration as a default for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#set-a-code-security-configuration-as-a-default-for-an-enterprise)
    pub async fn set_defaults(
        &self,
        configuration_id: u64,
        body: &impl serde::Serialize,
    ) -> Result<CodeSecurityDefaultConfiguration> {
        let route = format!(
            "/enterprises/{}/code-security/configurations/{}/defaults",
            self.handler.enterprise, configuration_id
        );
        self.handler.crab.put(route, Some(body)).await
    }

    /// Get repositories associated with an enterprise code security configuration.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-security/configurations?apiVersion=2022-11-28#get-repositories-associated-with-an-enterprise-code-security-configuration)
    pub fn repositories<'c>(
        &'c self,
        configuration_id: u64,
    ) -> ListEnterpriseCodeSecurityConfigRepositoriesBuilder<'octo, 'b, 'c> {
        ListEnterpriseCodeSecurityConfigRepositoriesBuilder::new(self, configuration_id)
    }
}

/// Builder for listing enterprise code security configurations.
#[derive(serde::Serialize)]
pub struct ListEnterpriseCodeSecurityConfigsBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c EnterpriseCodeSecurityHandler<'octo, 'b>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
}

impl<'octo, 'b, 'c> ListEnterpriseCodeSecurityConfigsBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c EnterpriseCodeSecurityHandler<'octo, 'b>) -> Self {
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
            "/enterprises/{}/code-security/configurations",
            self.handler.handler.enterprise
        );
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing repositories associated with an enterprise code security configuration.
#[derive(serde::Serialize)]
pub struct ListEnterpriseCodeSecurityConfigRepositoriesBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c EnterpriseCodeSecurityHandler<'octo, 'b>,
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

impl<'octo, 'b, 'c> ListEnterpriseCodeSecurityConfigRepositoriesBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(
        handler: &'c EnterpriseCodeSecurityHandler<'octo, 'b>,
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
            "/enterprises/{}/code-security/configurations/{}/repositories",
            self.handler.handler.enterprise, self.configuration_id
        );
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
