//! Organization Codespaces API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28)

use http::StatusCode;

use crate::api::codespaces::CodespacesSecretRepositoriesHandler;
use crate::models::codespaces::{Codespace, OrgCodespacesAccess, OrgCodespacesAccessVisibility};
use crate::models::orgs::secrets::{
    CreateOrganizationSecret, CreateOrganizationSecretResponse, OrganizationSecret,
};
use crate::models::PublicKey;
use crate::{Octocrab, Page, Result};

/// Client for GitHub's organization Codespaces API.
///
/// Created with [`crate::api::orgs::OrgHandler::codespaces`].
pub struct OrgCodespacesHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgCodespacesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Lists codespaces for the organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#list-codespaces-for-the-organization)
    pub fn list(&self) -> ListOrgCodespacesBuilder<'octo, '_> {
        ListOrgCodespacesBuilder::new(self)
    }

    /// Gets access control configuration for organization codespaces.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#manage-access-control-for-organization-codespaces)
    pub async fn access(&self) -> Result<OrgCodespacesAccess> {
        let route = format!("/orgs/{}/codespaces/access", self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// Updates access control configuration for organization codespaces.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#manage-access-control-for-organization-codespaces)
    pub async fn update_access(
        &self,
        visibility: OrgCodespacesAccessVisibility,
        selected_usernames: Option<&[impl AsRef<str>]>,
    ) -> Result<()> {
        let route = format!("/orgs/{}/codespaces/access", self.owner);
        #[derive(serde::Serialize)]
        struct Body<'a> {
            visibility: OrgCodespacesAccessVisibility,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_usernames: Option<Vec<&'a str>>,
        }

        let body = Body {
            visibility,
            selected_usernames: selected_usernames
                .map(|users| users.iter().map(AsRef::as_ref).collect()),
        };

        let resp = self.crab._put(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Adds users to Codespaces access for an organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#add-users-to-codespaces-access-for-an-organization)
    pub async fn add_selected_users(&self, selected_usernames: &[impl AsRef<str>]) -> Result<()> {
        let route = format!("/orgs/{}/codespaces/access/selected_users", self.owner);
        let body = serde_json::json!({
            "selected_usernames": selected_usernames.iter().map(AsRef::as_ref).collect::<Vec<_>>()
        });
        let resp = self.crab._post(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Removes users from Codespaces access for an organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#remove-users-from-codespaces-access-for-an-organization)
    pub async fn remove_selected_users(
        &self,
        selected_usernames: &[impl AsRef<str>],
    ) -> Result<()> {
        let route = format!("/orgs/{}/codespaces/access/selected_users", self.owner);
        let body = serde_json::json!({
            "selected_usernames": selected_usernames.iter().map(AsRef::as_ref).collect::<Vec<_>>()
        });
        let resp = self.crab._delete(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Lists codespaces for a user in the organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#list-codespaces-for-a-user-in-organization)
    pub fn list_for_user(
        &self,
        username: impl Into<String>,
    ) -> ListUserOrgCodespacesBuilder<'octo, '_> {
        ListUserOrgCodespacesBuilder::new(self, username.into())
    }

    /// Deletes a codespace for a user in the organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#delete-a-codespace-from-the-organization)
    pub async fn delete_for_user(
        &self,
        username: impl AsRef<str>,
        codespace_name: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/members/{}/codespaces/{}",
            self.owner,
            username.as_ref(),
            codespace_name.as_ref()
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Stops a codespace for an organization user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organizations?apiVersion=2022-11-28#stop-a-codespace-for-an-organization-user)
    pub async fn stop_for_user(
        &self,
        username: impl AsRef<str>,
        codespace_name: impl AsRef<str>,
    ) -> Result<Codespace> {
        let route = format!(
            "/orgs/{}/members/{}/codespaces/{}/stop",
            self.owner,
            username.as_ref(),
            codespace_name.as_ref()
        );
        self.crab.post(route, None::<&()>).await
    }

    /// Handler for managing organization Codespaces secrets.
    pub fn secrets(&self) -> OrgCodespacesSecretsHandler<'octo> {
        OrgCodespacesSecretsHandler::new(self.crab, self.owner.clone())
    }
}

/// Builder for listing organization codespaces.
#[derive(serde::Serialize)]
pub struct ListOrgCodespacesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgCodespacesHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgCodespacesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgCodespacesHandler<'octo>) -> Self {
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

    /// Sends the request and returns the resulting page of codespaces.
    pub async fn send(self) -> Result<Page<Codespace>> {
        let route = format!("/orgs/{}/codespaces", self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing organization codespaces for a user.
#[derive(serde::Serialize)]
pub struct ListUserOrgCodespacesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgCodespacesHandler<'octo>,
    #[serde(skip)]
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListUserOrgCodespacesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgCodespacesHandler<'octo>, username: String) -> Self {
        Self {
            handler,
            username,
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

    /// Sends the request and returns the resulting page of codespaces.
    pub async fn send(self) -> Result<Page<Codespace>> {
        let route = format!(
            "/orgs/{}/members/{}/codespaces",
            self.handler.owner, self.username
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for organization Codespaces secrets.
pub struct OrgCodespacesSecretsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgCodespacesSecretsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Lists organization Codespaces secrets.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organization-secrets?apiVersion=2022-11-28#list-organization-secrets)
    pub fn list(&self) -> ListOrgCodespacesSecretsBuilder<'octo, '_> {
        ListOrgCodespacesSecretsBuilder::new(self)
    }

    /// Gets the organization public key for Codespaces secrets encryption.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organization-secrets?apiVersion=2022-11-28#get-an-organization-public-key)
    pub async fn get_public_key(&self) -> Result<PublicKey> {
        let route = format!("/orgs/{}/codespaces/secrets/public-key", self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// Gets an organization Codespaces secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organization-secrets?apiVersion=2022-11-28#get-an-organization-secret)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<OrganizationSecret> {
        let route = format!(
            "/orgs/{}/codespaces/secrets/{}",
            self.owner,
            secret_name.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Creates or updates an organization Codespaces secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organization-secrets?apiVersion=2022-11-28#create-or-update-an-organization-secret)
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateOrganizationSecret<'_>,
    ) -> Result<CreateOrganizationSecretResponse> {
        let route = format!(
            "/orgs/{}/codespaces/secrets/{}",
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

    /// Deletes an organization Codespaces secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/organization-secrets?apiVersion=2022-11-28#delete-an-organization-secret)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/codespaces/secrets/{}",
            self.owner,
            secret_name.as_ref()
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Manages repository access for an organization codespaces secret.
    pub fn repositories(
        &self,
        secret_name: impl AsRef<str>,
    ) -> CodespacesSecretRepositoriesHandler<'octo> {
        CodespacesSecretRepositoriesHandler::new(
            self.crab,
            format!(
                "/orgs/{}/codespaces/secrets/{}",
                self.owner,
                secret_name.as_ref()
            ),
        )
    }
}

/// Builder for listing organization codespaces secrets.
#[derive(serde::Serialize)]
pub struct ListOrgCodespacesSecretsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgCodespacesSecretsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgCodespacesSecretsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgCodespacesSecretsHandler<'octo>) -> Self {
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
        let route = format!("/orgs/{}/codespaces/secrets", self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}
