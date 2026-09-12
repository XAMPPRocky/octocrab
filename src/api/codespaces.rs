//! GitHub Codespaces API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces?apiVersion=2022-11-28)

use http::StatusCode;

use crate::models::codespaces::{
    Codespace, CodespaceExportDetails, CodespaceMachine, CodespacesSecret, CreateUserCodespace,
    CreateUserCodespacesSecret, PublishCodespace, SelectedRepositories, SetSelectedRepositories,
    UpdateCodespace,
};
use crate::models::orgs::secrets::CreateOrganizationSecretResponse;
use crate::models::{PublicKey, RepositoryId};
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's Codespaces API.
///
/// Created with [`Octocrab::codespaces`].
pub struct CodespacesHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CodespacesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Lists codespaces for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#list-codespaces-for-the-authenticated-user)
    pub fn list(&self) -> ListCodespacesBuilder<'octo, '_> {
        ListCodespacesBuilder::new(self)
    }

    /// Creates a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#create-a-codespace-for-the-authenticated-user)
    pub async fn create(&self, body: &CreateUserCodespace) -> Result<Codespace> {
        self.crab.post("/user/codespaces", Some(body)).await
    }

    /// Gets a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#get-a-codespace-for-the-authenticated-user)
    pub async fn get(&self, codespace_name: impl AsRef<str>) -> Result<Codespace> {
        let route = format!("/user/codespaces/{}", codespace_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Updates a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#update-a-codespace-for-the-authenticated-user)
    pub async fn update(
        &self,
        codespace_name: impl AsRef<str>,
        patch: &UpdateCodespace,
    ) -> Result<Codespace> {
        let route = format!("/user/codespaces/{}", codespace_name.as_ref());
        self.crab.patch(route, Some(patch)).await
    }

    /// Deletes a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#delete-a-codespace-for-the-authenticated-user)
    pub async fn delete(&self, codespace_name: impl AsRef<str>) -> Result<()> {
        let route = format!("/user/codespaces/{}", codespace_name.as_ref());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Starts a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#start-a-codespace-for-the-authenticated-user)
    pub async fn start(&self, codespace_name: impl AsRef<str>) -> Result<Codespace> {
        let route = format!("/user/codespaces/{}/start", codespace_name.as_ref());
        self.crab.post(route, None::<&()>).await
    }

    /// Stops a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#stop-a-codespace-for-the-authenticated-user)
    pub async fn stop(&self, codespace_name: impl AsRef<str>) -> Result<Codespace> {
        let route = format!("/user/codespaces/{}/stop", codespace_name.as_ref());
        self.crab.post(route, None::<&()>).await
    }

    /// Creates a repository from an unpublished codespace.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#create-a-repository-from-an-unpublished-codespace)
    pub async fn publish(
        &self,
        codespace_name: impl AsRef<str>,
        body: &PublishCodespace,
    ) -> Result<Codespace> {
        let route = format!("/user/codespaces/{}/publish", codespace_name.as_ref());
        self.crab.post(route, Some(body)).await
    }

    /// Triggers an export of a codespace for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#export-a-codespace-for-the-authenticated-user)
    pub async fn export(&self, codespace_name: impl AsRef<str>) -> Result<CodespaceExportDetails> {
        let route = format!("/user/codespaces/{}/exports", codespace_name.as_ref());
        self.crab.post(route, None::<&()>).await
    }

    /// Gets details about a codespace export.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#get-details-about-a-codespace-export)
    pub async fn get_export(
        &self,
        codespace_name: impl AsRef<str>,
        export_id: impl AsRef<str>,
    ) -> Result<CodespaceExportDetails> {
        let route = format!(
            "/user/codespaces/{}/exports/{}",
            codespace_name.as_ref(),
            export_id.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Lists machine types available for a codespace.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/machines?apiVersion=2022-11-28#list-machine-types-for-a-codespace)
    pub async fn machines(
        &self,
        codespace_name: impl AsRef<str>,
    ) -> Result<Page<CodespaceMachine>> {
        let route = format!("/user/codespaces/{}/machines", codespace_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Handler for managing user-scoped Codespaces secrets.
    pub fn secrets(&self) -> UserCodespacesSecretsHandler<'octo> {
        UserCodespacesSecretsHandler::new(self.crab)
    }
}

/// Builder for listing codespaces for the authenticated user.
#[derive(serde::Serialize)]
pub struct ListCodespacesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r CodespacesHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCodespacesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r CodespacesHandler<'octo>) -> Self {
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
        self.handler.crab.get("/user/codespaces", Some(&self)).await
    }
}

/// Handler for Codespaces secrets of the authenticated user.
pub struct UserCodespacesSecretsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> UserCodespacesSecretsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Lists development environment secrets for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/secrets?apiVersion=2022-11-28#list-secrets-for-the-authenticated-user)
    pub fn list(&self) -> ListUserCodespacesSecretsBuilder<'octo, '_> {
        ListUserCodespacesSecretsBuilder::new(self)
    }

    /// Gets the public key for encrypting secrets for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/secrets?apiVersion=2022-11-28#get-public-key-for-the-authenticated-user)
    pub async fn get_public_key(&self) -> Result<PublicKey> {
        self.crab
            .get("/user/codespaces/secrets/public-key", None::<&()>)
            .await
    }

    /// Gets a development environment secret for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/secrets?apiVersion=2022-11-28#get-a-secret-for-the-authenticated-user)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<CodespacesSecret> {
        let route = format!("/user/codespaces/secrets/{}", secret_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates or updates a development environment secret for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/secrets?apiVersion=2022-11-28#create-or-update-a-secret-for-the-authenticated-user)
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateUserCodespacesSecret<'_>,
    ) -> Result<CreateOrganizationSecretResponse> {
        let route = format!("/user/codespaces/secrets/{}", secret_name.as_ref());
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

    /// Deletes a development environment secret for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/secrets?apiVersion=2022-11-28#delete-a-secret-for-the-authenticated-user)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!("/user/codespaces/secrets/{}", secret_name.as_ref());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Manages repository access for a user codespaces secret.
    pub fn repositories(
        &self,
        secret_name: impl AsRef<str>,
    ) -> CodespacesSecretRepositoriesHandler<'octo> {
        CodespacesSecretRepositoriesHandler::new(
            self.crab,
            format!("/user/codespaces/secrets/{}", secret_name.as_ref()),
        )
    }
}

/// Builder for listing user codespaces secrets.
#[derive(serde::Serialize)]
pub struct ListUserCodespacesSecretsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r UserCodespacesSecretsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListUserCodespacesSecretsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r UserCodespacesSecretsHandler<'octo>) -> Self {
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
    pub async fn send(self) -> Result<Page<CodespacesSecret>> {
        self.handler
            .crab
            .get("/user/codespaces/secrets", Some(&self))
            .await
    }
}

/// Shared handler for managing selected repositories for Codespaces secrets.
pub struct CodespacesSecretRepositoriesHandler<'octo> {
    crab: &'octo Octocrab,
    route_prefix: String,
}

impl<'octo> CodespacesSecretRepositoriesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, route_prefix: String) -> Self {
        Self { crab, route_prefix }
    }

    /// Lists selected repositories for a secret.
    pub async fn list(&self) -> Result<SelectedRepositories> {
        let route = format!("{}/repositories", self.route_prefix);
        self.crab.get(route, None::<&()>).await
    }

    /// Sets selected repositories for a secret.
    pub async fn set(&self, repository_ids: &[RepositoryId]) -> Result<()> {
        let route = format!("{}/repositories", self.route_prefix);
        let body = SetSelectedRepositories {
            selected_repository_ids: repository_ids,
        };
        let resp = self.crab._put(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Adds a selected repository to a secret.
    pub async fn add(&self, repository_id: RepositoryId) -> Result<()> {
        let route = format!("{}/repositories/{}", self.route_prefix, repository_id);
        let resp = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Removes a selected repository from a secret.
    pub async fn remove(&self, repository_id: RepositoryId) -> Result<()> {
        let route = format!("{}/repositories/{}", self.route_prefix, repository_id);
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}
