use http::StatusCode;

use super::RepoHandler;
use crate::models::repos::variables::{CreateRepositoryVariable, CreateRepositoryVariableResponse};

/// A client to GitHub's repository variables API.
///
/// Created with [`RepoHandler`].
#[derive(serde::Serialize)]
pub struct RepoVariablesHandler<'octo> {
    #[serde(skip)]
    handler: &'octo RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> RepoVariablesHandler<'octo> {
    pub(crate) fn new(repo: &'octo RepoHandler<'octo>) -> Self {
        Self {
            handler: repo,
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

    /// Lists all repository variables.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth app tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///
    /// let all_variables = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .list()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn list(
        &self,
    ) -> crate::Result<crate::models::repos::variables::RepositoryVariables> {
        let route = format!("/{}/actions/variables", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Gets a specific variable in a repository.
    /// The authenticated user must have collaborator access to the repository to use this endpoint.
    /// OAuth app tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let variable = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .get("EMAIL")
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn get(
        &self,
        variable_name: impl AsRef<str>,
    ) -> crate::Result<crate::models::repos::variables::RepositoryVariable> {
        let route = format!(
            "/{}/actions/variables/{variable_name}",
            self.handler.repo,
            variable_name = variable_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates or updates a repository variable that you can reference in a GitHub Actions workflow.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models::repos::variables::{CreateRepositoryVariable, CreateRepositoryVariableResponse};
    ///
    /// let result = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .create_or_update(&CreateRepositoryVariable{
    ///         name: "GH_TOKEN",
    ///         value: "octocat@github.com",
    ///     })
    ///     .await?;
    ///
    /// match result {
    ///    CreateRepositoryVariableResponse::Created => println!("Created variable!"),
    ///    CreateRepositoryVariableResponse::Updated => println!("Updated variable!"),
    /// }
    ///
    /// # Ok(())
    /// # }
    pub async fn create_or_update(
        &self,
        variable: &CreateRepositoryVariable<'_>,
    ) -> crate::Result<CreateRepositoryVariableResponse> {
        let route = format!(
            "/{}/actions/variables/{variable_name}",
            self.handler.repo,
            variable_name = variable.name
        );

        let resp = {
            let resp = self.handler.crab._put(route, Some(variable)).await?;
            crate::map_github_error(resp).await?
        };

        match resp.status() {
            StatusCode::CREATED => Ok(CreateRepositoryVariableResponse::Created),
            StatusCode::NO_CONTENT => Ok(CreateRepositoryVariableResponse::Updated),
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

    /// Deletes a repository variable using the variable name.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let repo = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .delete_variable("EMAIL")
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn delete(&self, variable_name: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/{}/actions/variables/{variable_name}",
            self.handler.repo,
            variable_name = variable_name.as_ref()
        );

        let resp = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}
