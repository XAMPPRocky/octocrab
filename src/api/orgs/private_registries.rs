use super::OrgHandler;
use crate::models::private_registries::{
    OrgPrivateRegistriesResponse, OrgPrivateRegistry, OrgPrivateRegistryPublicKey,
};
use crate::Result;

/// A client to GitHub's organization Private Registries API.
///
/// Created with [`OrgHandler::private_registries`].
pub struct OrgPrivateRegistriesHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgPrivateRegistriesHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List private registries for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/private-registries/organization-configurations?apiVersion=2022-11-28#list-private-registries-for-an-organization)
    pub fn list<'c>(&'c self) -> ListOrgPrivateRegistriesBuilder<'octo, 'b, 'c> {
        ListOrgPrivateRegistriesBuilder::new(self)
    }

    /// Get a private registry for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/private-registries/organization-configurations?apiVersion=2022-11-28#get-a-private-registry-for-an-organization)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<OrgPrivateRegistry> {
        let route = format!(
            "/orgs/{}/private-registries/{}",
            self.handler.owner,
            secret_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create a private registry for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/private-registries/organization-configurations?apiVersion=2022-11-28#create-a-private-registry-for-an-organization)
    pub async fn create(&self, config: &impl serde::Serialize) -> Result<OrgPrivateRegistry> {
        let route = format!("/orgs/{}/private-registries", self.handler.owner);
        self.handler.crab.post(route, Some(config)).await
    }

    /// Update a private registry for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/private-registries/organization-configurations?apiVersion=2022-11-28#update-a-private-registry-for-an-organization)
    pub async fn update(
        &self,
        secret_name: impl AsRef<str>,
        config: &impl serde::Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/private-registries/{}",
            self.handler.owner,
            secret_name.as_ref()
        );
        let response = self.handler.crab._patch(route, Some(config)).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Delete a private registry for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/private-registries/organization-configurations?apiVersion=2022-11-28#delete-a-private-registry-for-an-organization)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/private-registries/{}",
            self.handler.owner,
            secret_name.as_ref()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Get private registries public key for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/private-registries/organization-configurations?apiVersion=2022-11-28#get-private-registries-public-key-for-an-organization)
    pub async fn public_key(&self) -> Result<OrgPrivateRegistryPublicKey> {
        let route = format!("/orgs/{}/private-registries/public-key", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }
}

/// Builder for listing organization private registries.
#[derive(serde::Serialize)]
pub struct ListOrgPrivateRegistriesBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c OrgPrivateRegistriesHandler<'octo, 'b>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b, 'c> ListOrgPrivateRegistriesBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c OrgPrivateRegistriesHandler<'octo, 'b>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> Result<OrgPrivateRegistriesResponse> {
        let route = format!("/orgs/{}/private-registries", self.handler.handler.owner);
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
