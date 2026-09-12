use http::StatusCode;
use serde::Serialize;

use crate::{
    actions::ActionsHandler,
    models::{
        actions::{
            CreateEnvironmentSecret, CreateEnvironmentSecretResponse, EnvironmentSecret,
            EnvironmentSecrets, EnvironmentVariable, EnvironmentVariables,
        },
        PublicKey, RepositoryId,
    },
    Octocrab, Result,
};

/// Handler for GitHub Actions environment secrets.
#[derive(Serialize)]
pub struct EnvironmentSecretsHandler<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repository_id: RepositoryId,
    #[serde(skip)]
    environment_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> EnvironmentSecretsHandler<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        repository_id: RepositoryId,
        environment_name: impl Into<String>,
    ) -> Self {
        Self {
            crab,
            repository_id,
            environment_name: environment_name.into(),
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

    fn base_route(&self) -> String {
        format!(
            "/repositories/{}/environments/{}/secrets",
            self.repository_id, self.environment_name
        )
    }

    /// Gets the public key for an environment, which you need to encrypt secrets.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/secrets?apiVersion=2022-11-28#get-an-environment-public-key)
    pub async fn get_public_key(&self) -> Result<PublicKey> {
        let route = format!("{}/public-key", self.base_route());
        self.crab.get(route, None::<&()>).await
    }

    /// Lists secrets available in an environment without revealing their encrypted values.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/secrets?apiVersion=2022-11-28#list-environment-secrets)
    pub async fn list(&self) -> Result<EnvironmentSecrets> {
        let route = self.base_route();
        self.crab.get(route, Some(&self)).await
    }

    /// Gets a single environment secret without revealing its encrypted value.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/secrets?apiVersion=2022-11-28#get-an-environment-secret)
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<EnvironmentSecret> {
        let route = format!("{}/{}", self.base_route(), secret_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates or updates an environment secret with an encrypted value.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/secrets?apiVersion=2022-11-28#create-or-update-an-environment-secret)
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateEnvironmentSecret<'_>,
    ) -> Result<CreateEnvironmentSecretResponse> {
        let route = format!("{}/{}", self.base_route(), secret_name.as_ref());
        let resp = {
            let resp = self.crab._put(route, Some(secret)).await?;
            crate::map_github_error(resp).await?
        };

        match resp.status() {
            StatusCode::CREATED => Ok(CreateEnvironmentSecretResponse::Created),
            StatusCode::NO_CONTENT => Ok(CreateEnvironmentSecretResponse::Updated),
            status_code => Err(crate::Error::Other {
                source: format!(
                    "Unexpected status code from request: {}",
                    status_code.as_str()
                )
                .into(),
                backtrace: snafu::Backtrace::capture(),
            }),
        }
    }

    /// Deletes an environment secret.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/secrets?apiVersion=2022-11-28#delete-an-environment-secret)
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!("{}/{}", self.base_route(), secret_name.as_ref());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// Handler for GitHub Actions environment variables.
#[derive(Serialize)]
pub struct EnvironmentVariablesHandler<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repository_id: RepositoryId,
    #[serde(skip)]
    environment_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> EnvironmentVariablesHandler<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        repository_id: RepositoryId,
        environment_name: impl Into<String>,
    ) -> Self {
        Self {
            crab,
            repository_id,
            environment_name: environment_name.into(),
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

    fn base_route(&self) -> String {
        format!(
            "/repositories/{}/environments/{}/variables",
            self.repository_id, self.environment_name
        )
    }

    /// Lists all environment variables.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#list-environment-variables)
    pub async fn list(&self) -> Result<EnvironmentVariables> {
        let route = self.base_route();
        self.crab.get(route, Some(&self)).await
    }

    /// Gets a specific environment variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#get-an-environment-variable)
    pub async fn get(&self, name: impl AsRef<str>) -> Result<EnvironmentVariable> {
        let route = format!("{}/{}", self.base_route(), name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates an environment variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#create-an-environment-variable)
    pub async fn create(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        let route = self.base_route();
        let body = serde_json::json!({
            "name": name.as_ref(),
            "value": value.as_ref(),
        });
        let resp = self.crab._post(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Updates an environment variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#update-an-environment-variable)
    pub async fn update(
        &self,
        name: impl AsRef<str>,
        new_name: Option<impl AsRef<str>>,
        value: Option<impl AsRef<str>>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<&'a str>,
        }
        let route = format!("{}/{}", self.base_route(), name.as_ref());
        let body = Body {
            name: new_name.as_ref().map(AsRef::as_ref),
            value: value.as_ref().map(AsRef::as_ref),
        };
        let resp = self.crab._patch(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Deletes an environment variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#delete-an-environment-variable)
    pub async fn delete(&self, name: impl AsRef<str>) -> Result<()> {
        let route = format!("{}/{}", self.base_route(), name.as_ref());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

impl<'octo> ActionsHandler<'octo> {
    /// Accesses environment secrets API for a repository and environment.
    pub fn environment_secrets(
        &self,
        repository_id: RepositoryId,
        environment_name: impl Into<String>,
    ) -> EnvironmentSecretsHandler<'octo> {
        EnvironmentSecretsHandler::new(self.crab, repository_id, environment_name)
    }

    /// Accesses environment variables API for a repository and environment.
    pub fn environment_variables(
        &self,
        repository_id: RepositoryId,
        environment_name: impl Into<String>,
    ) -> EnvironmentVariablesHandler<'octo> {
        EnvironmentVariablesHandler::new(self.crab, repository_id, environment_name)
    }
}
