use crate::error::HttpSnafu;
use crate::params;
use crate::{models, FromResponse, Octocrab, Page, Result};
use http::header::ACCEPT;
use http::request::Builder;
use http::{StatusCode, Uri};
use snafu::ResultExt;

#[derive(Debug, serde::Serialize)]
struct PermissionUpdateBody {
    permission: params::teams::Permission,
}

/// Handler for managing a team's repositories through
/// GitHub's teams API.
///
/// Created with [`TeamHandler::repos`]
///
/// [`TeamHandler::repos`]: ./struct.TeamHandler.html#method.repos
pub struct TeamRepoHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
    team: String,
}

impl<'octo> TeamRepoHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String, team: String) -> Self {
        Self { crab, org, team }
    }

    /// Lists the repositories a team has access to.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let repos = octocrab::instance()
    ///     .teams("owner")
    ///     .repos("team")
    ///     .list()
    ///     .per_page(10)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListTeamRepositoriesBuilder<'octo, '_> {
        ListTeamRepositoriesBuilder::new(self)
    }

    /// Checks if a team manages a repository, returning the repository if it does.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let manages_repo = octocrab::instance()
    ///     .teams("owner")
    ///     .repos("team")
    ///     .check_manages("owner", "repo")
    ///     .await
    ///     .is_ok();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_manages(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
    ) -> Result<Option<models::Repository>> {
        let route = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let request = Builder::new()
            .method("GET")
            .uri(uri)
            .header(ACCEPT, crate::format_media_type("inertia-preview+json"));
        let request = self.crab.build_request(request, None::<&()>)?;

        let res = self.crab.execute(request).await?;
        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        Ok(Some(models::Repository::from_response(res).await?))
    }

    /// Updates a team's permissions for a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .repos("team")
    ///     .add_or_update("owner", "repo", params::teams::Permission::Maintain)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_or_update(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
        permission: impl Into<Option<params::teams::Permission>>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        let perm_body = permission
            .into()
            .map(|p| PermissionUpdateBody { permission: p });

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._put(uri, perm_body.as_ref()).await?)
            .await
            .map(drop)
    }

    /// Removes a repository from a team.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .repos("team")
    ///     .remove("owner", "repo")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        self.crab
            ._delete(
                self.crab.parameterized_uri(route, None::<&()>)?,
                None::<&()>,
            )
            .await?;
        Ok(())
    }
}

/// A builder pattern struct for listing the repositories of a team.
///
/// Created by [`TeamRepoHandler::list`].
///
/// [`TeamRepoHandler::list`]: ./struct.TeamRepoHandler.html#method.list
#[derive(serde::Serialize)]
pub struct ListTeamRepositoriesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamRepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListTeamRepositoriesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r TeamRepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page.
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::Repository>> {
        let route = format!(
            "/orgs/{org}/teams/{team}/repos",
            org = self.handler.org,
            team = self.handler.team,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
