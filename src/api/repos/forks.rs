use super::*;

/// The available methods to sort repository forks by.
#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ForkSort {
    Newest,
    Oldest,
    Stargazers,
}

#[derive(serde::Serialize)]
pub struct ListForksBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<ForkSort>,
}

impl<'octo, 'r> ListForksBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            sort: None,
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

    /// Sort order of the results.
    pub fn sort(mut self, sort: ForkSort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Sends the actual request.
    pub async fn send(
        self,
    ) -> crate::Result<crate::Page<crate::models::Repository>> {
        let url = format!(
            "/repos/{owner}/{repo}/forks",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}
#[derive(serde::Serialize)]
pub struct CreateForkBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<String>,
}
impl<'octo, 'r> CreateForkBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            organization: None,
        }
    }

    /// The organization name, if forking into an organization.
    pub fn organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::Repository> {
        let url = format!(
            "/repos/{owner}/{repo}/forks",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.post(url, Some(&self)).await
    }
}

impl<'octo> RepoHandler<'octo> {
    /// List forks of a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let forks = octocrab::instance().repos("owner", "repo").list_forks().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    /// Optionally, specify a sort order with [`ForkSort`].
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::repos::forks::ForkSort;
    /// let forks = octocrab::instance().repos("owner", "repo").list_forks().sort(ForkSort::Oldest).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_forks(&self) -> ListForksBuilder<'_, '_> {
        ListForksBuilder::new(self)
    }

    /// Creates a fork of a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let new_fork = octocrab::instance().repos("owner", "repo").create_fork().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_fork(&self) -> CreateForkBuilder<'_, '_> {
        CreateForkBuilder::new(self)
    }
}
