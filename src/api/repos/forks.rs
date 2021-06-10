use super::params::repos::forks::Sort;
use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListForksBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
    /// Sort order of the results.
    sort: Option<Sort>,
}

impl<'octo, 'r> ListForksBuilder<'octo, 'r> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Repository>> {
        let url = format!(
            "repos/{owner}/{repo}/forks",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct CreateForkBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    /// The organization name, if forking into an organization.
    organization: Option<String>,
}
impl<'octo, 'r> CreateForkBuilder<'octo, 'r> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::Repository> {
        let url = format!(
            "repos/{owner}/{repo}/forks",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.post(url, Some(&self)).await
    }
}

impl<'octo> RepoHandler<'octo> {
    /// List forks of a repository. Optionally, specify the
    /// [sort](ListForksBuilder::sort()) order,
    /// [page](ListForksBuilder::page()),
    /// and items [per_page](ListForksBuilder::per_page())
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::repos::forks::Sort;
    /// let forks = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .list_forks()
    ///     // Optional Parameters
    ///     .sort(Sort::Oldest)
    ///     .page(2u32)
    ///     .per_page(30)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_forks(&self) -> ListForksBuilder<'_, '_> {
        ListForksBuilder::new(self)
    }

    /// Creates a fork of a repository. Optionally, specify the target
    /// [organization](CreateForkBuilder::organization()) to
    /// create the fork in.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let new_fork = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .create_fork()
    ///     // Optional Parameters
    ///     .organization("weyland-yutani")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_fork(&self) -> CreateForkBuilder<'_, '_> {
        CreateForkBuilder::new(self)
    }
}
