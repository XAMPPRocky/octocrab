use crate::params;
use crate::{models, FromResponse, Octocrab, Result};
use http::{header::ACCEPT, StatusCode};

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
        let url = format!(
            "orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        let req = self
            .crab
            .client
            .get(self.crab.absolute_url(&url)?)
            .header(ACCEPT, "application/vnd.github.v3.repository+json");
        let res = self.crab.execute(req).await?;
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
        let url = format!(
            "orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        let perm_body = permission
            .into()
            .map(|p| PermissionUpdateBody { permission: p });
        crate::map_github_error(
            self.crab
                ._put(self.crab.absolute_url(&url)?, perm_body.as_ref())
                .await?,
        )
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
        let url = format!(
            "orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        self.crab.delete(url, None::<&()>).await
    }
}
