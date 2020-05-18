use crate::params;
use crate::{Octocrab, Result};
use reqwest::StatusCode;

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

/// Whether a team manages a repository.
pub enum ManagesRepo {
    Yes,
    No,
    Err,
}

impl<'octo> TeamRepoHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String, team: String) -> Self {
        Self { crab, org, team }
    }

    /// Checks if a team manages a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let manages_repo = octocrab.teams("owner")
    ///     .repos("team")
    ///     .manages("owner", "repo")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn manages(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
    ) -> Result<ManagesRepo> {
        let url = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        let res = self.crab._get(&url, None::<&()>).await?;
        Ok(match res.status() {
            StatusCode::NO_CONTENT => ManagesRepo::Yes,
            StatusCode::NOT_FOUND => ManagesRepo::No,
            _ => ManagesRepo::Err,
        })
    }

    /// Updates a team's permissions for a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    /// octocrab.teams("owner")
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
        permission: Option<params::teams::Permission>,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        self.crab.put(url, permission.as_ref()).await
    }

    /// Removes a repository from a team.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.teams("owner")
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
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        self.crab.delete(url, None::<&()>).await
    }
}
