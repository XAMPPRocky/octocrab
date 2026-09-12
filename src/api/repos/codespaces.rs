//! Repository Codespaces API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces?apiVersion=2022-11-28)

use http::StatusCode;

use crate::api::repos::RepoRef;
use crate::models::codespaces::{
    Codespace, CodespaceDefaultAttributes, CodespaceMachine, CodespacePermissionsCheck,
    CreatePullRequestCodespace, CreateRepoCodespace, Devcontainer,
};
use crate::models::repos::secrets::{
    CreateRepositorySecret, CreateRepositorySecretResponse, RepositorySecret,
};
use crate::models::PublicKey;
use crate::{Octocrab, Page, Result};

/// Client for GitHub's repository Codespaces API.
///
/// Created with [`crate::api::repos::RepoHandler::codespaces`].
pub struct RepoCodespacesHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoCodespacesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Lists codespaces in a repository for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#list-codespaces-in-a-repository-for-the-authenticated-user)
    pub fn list(&self) -> ListRepoCodespacesBuilder<'octo, '_> {
        ListRepoCodespacesBuilder::new(self)
    }

    /// Creates a codespace in a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#create-a-codespace-in-a-repository)
    pub async fn create(&self, body: &CreateRepoCodespace) -> Result<Codespace> {
        let route = format!("/{}/codespaces", self.repo);
        self.crab.post(route, Some(body)).await
    }

    /// Creates a codespace from a pull request.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#create-a-codespace-from-a-pull-request)
    pub async fn create_for_pull_request(
        &self,
        pull_number: u64,
        body: &CreatePullRequestCodespace,
    ) -> Result<Codespace> {
        let route = format!("/{}/pulls/{}/codespaces", self.repo, pull_number);
        self.crab.post(route, Some(body)).await
    }

    /// Lists devcontainer configurations in a repository for the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#list-devcontainer-configurations-in-a-repository-for-the-authenticated-user)
    pub fn devcontainers(&self) -> ListDevcontainersBuilder<'octo, '_> {
        ListDevcontainersBuilder::new(self)
    }

    /// Gets default attributes for a codespace created in this repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#get-default-attributes-for-a-codespace)
    pub fn default_attributes(&self) -> GetDefaultAttributesBuilder<'octo, '_> {
        GetDefaultAttributesBuilder::new(self)
    }

    /// Checks if permissions defined by a devcontainer have been accepted by the authenticated user.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/codespaces?apiVersion=2022-11-28#check-if-permissions-defined-by-a-devcontainer-have-been-accepted-by-the-authenticated-user)
    pub async fn check_permissions(
        &self,
        ref_: impl AsRef<str>,
        devcontainer_path: impl AsRef<str>,
    ) -> Result<bool> {
        let route = format!("/{}/codespaces/permissions_check", self.repo);
        #[derive(serde::Serialize)]
        struct Query<'a> {
            #[serde(rename = "ref")]
            ref_: &'a str,
            devcontainer_path: &'a str,
        }

        let query = Query {
            ref_: ref_.as_ref(),
            devcontainer_path: devcontainer_path.as_ref(),
        };

        let res: CodespacePermissionsCheck = self.crab.get(route, Some(&query)).await?;
        Ok(res.accepted)
    }

    /// Lists available machine types for a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/machines?apiVersion=2022-11-28#list-available-machine-types-for-a-repository)
    pub async fn machines(&self) -> Result<Page<CodespaceMachine>> {
        let route = format!("/{}/codespaces/machines", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Handler for managing repository Codespaces secrets.
    pub fn secrets(&self) -> RepoCodespacesSecretsHandler<'octo> {
        RepoCodespacesSecretsHandler::new(self.crab, self.repo.clone())
    }
}

/// Builder for listing codespaces in a repository for the authenticated user.
#[derive(serde::Serialize)]
pub struct ListRepoCodespacesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoCodespacesHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRepoCodespacesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoCodespacesHandler<'octo>) -> Self {
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
        let route = format!("/{}/codespaces", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing devcontainer configurations in a repository.
#[derive(serde::Serialize)]
pub struct ListDevcontainersBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoCodespacesHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListDevcontainersBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoCodespacesHandler<'octo>) -> Self {
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

    /// Sends the request and returns the resulting page of devcontainers.
    pub async fn send(self) -> Result<Page<Devcontainer>> {
        let route = format!("/{}/codespaces/devcontainers", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for getting default attributes for a codespace.
#[derive(serde::Serialize)]
pub struct GetDefaultAttributesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoCodespacesHandler<'octo>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ip: Option<String>,
}

impl<'octo, 'r> GetDefaultAttributesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoCodespacesHandler<'octo>) -> Self {
        Self {
            handler,
            ref_: None,
            client_ip: None,
        }
    }

    /// The branch or commit to check for a default devcontainer path.
    pub fn ref_(mut self, ref_: impl Into<String>) -> Self {
        self.ref_ = Some(ref_.into());
        self
    }

    /// An alternative IP for default location auto-detection.
    pub fn client_ip(mut self, client_ip: impl Into<String>) -> Self {
        self.client_ip = Some(client_ip.into());
        self
    }

    /// Sends the request and returns default attributes.
    pub async fn send(self) -> Result<CodespaceDefaultAttributes> {
        let route = format!("/{}/codespaces/new", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for repository Codespaces secrets.
pub struct RepoCodespacesSecretsHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoCodespacesSecretsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Lists repository Codespaces secrets.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/repository-secrets?apiVersion=2022-11-28#list-repository-secrets)
    pub fn list(&self) -> ListRepoCodespacesSecretsBuilder<'octo, '_> {
        ListRepoCodespacesSecretsBuilder::new(self)
    }

    /// Gets the repository public key for Codespaces secrets encryption.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/repository-secrets?apiVersion=2022-11-28#get-a-repository-public-key)
    pub async fn get_public_key(&self) -> Result<PublicKey> {
        let route = format!("/{}/codespaces/secrets/public-key", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Gets a repository Codespaces secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/repository-secrets?apiVersion=2022-11-28#get-a-repository-secret)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<RepositorySecret> {
        let route = format!("/{}/codespaces/secrets/{}", self.repo, secret_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates or updates a repository Codespaces secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/repository-secrets?apiVersion=2022-11-28#create-or-update-a-repository-secret)
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateRepositorySecret<'_>,
    ) -> Result<CreateRepositorySecretResponse> {
        let route = format!("/{}/codespaces/secrets/{}", self.repo, secret_name.as_ref());
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

    /// Deletes a repository Codespaces secret.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/codespaces/repository-secrets?apiVersion=2022-11-28#delete-a-repository-secret)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!("/{}/codespaces/secrets/{}", self.repo, secret_name.as_ref());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// Builder for listing repository codespaces secrets.
#[derive(serde::Serialize)]
pub struct ListRepoCodespacesSecretsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoCodespacesSecretsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRepoCodespacesSecretsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoCodespacesSecretsHandler<'octo>) -> Self {
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
        let route = format!("/{}/codespaces/secrets", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
