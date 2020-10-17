use super::*;

/// Handler for GitHub's releases API.
///
/// Created with [`RepoHandler::releases`].
///
/// [`RepoHandler::releases`]: ../struct.RepoHandler.html#method.releases
pub struct ReleasesHandler<'octo, 'r> {
    parent: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> ReleasesHandler<'octo, 'r> {
    pub(crate) fn new(parent: &'r RepoHandler<'octo>) -> Self {
        Self { parent }
    }

    /// Creates a new `ListPullRequestsBuilder` that can be configured to filter
    /// listing pulling requests.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let page = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .list()
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListReleasesBuilder<'_, '_, '_> {
        ListReleasesBuilder::new(self)
    }
}

/// A builder pattern struct for listing pull requests.
///
/// created by [`PullRequestHandler::list`]
///
/// [`PullRequestHandler::list`]: ./struct.PullRequestHandler.html#method.list
#[derive(serde::Serialize)]
pub struct ListReleasesBuilder<'octo, 'r1, 'r2> {
    #[serde(skip)]
    handler: &'r2 ReleasesHandler<'octo, 'r1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r1, 'r2> ListReleasesBuilder<'octo, 'r1, 'r2> {
    pub(crate) fn new(handler: &'r2 ReleasesHandler<'octo, 'r1>) -> Self {
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

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::Release>> {
        let url = format!(
            "/repos/{owner}/{repo}/releases",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.crab.get(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .head("master")
            .base("branch")
            .sort(crate::params::pulls::Sort::Popularity)
            .direction(crate::params::Direction::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "head": "master",
                "base": "branch",
                "sort": "popularity",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
