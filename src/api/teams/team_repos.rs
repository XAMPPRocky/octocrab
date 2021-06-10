use crate::params;
use crate::{models, FromResponse, Octocrab, Result};
use reqwest::header::ACCEPT;

/// Handler for managing a team's repositories through
/// GitHub's teams API.
///
/// Created with [`TeamHandler::repos`]
///
/// [`TeamHandler::repos`]: ./struct.TeamHandler.html#method.repos
#[derive(octocrab_derive::Builder)]
pub struct TeamRepoHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
    team: String,
}

impl<'octo> TeamRepoHandler<'octo> {
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
    ) -> Result<models::Repository> {
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
            .get(&url)
            .header(ACCEPT, "application/vnd.github.v3.repository+json");
        let res = self.crab.execute(req).await?;
        models::Repository::from_response(res).await
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
        crate::map_github_error(self.crab._put(&url, permission.into().as_ref()).await?)
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
