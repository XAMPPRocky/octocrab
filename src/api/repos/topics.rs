use super::RepoHandler;
use crate::models::repos::RepoTopics;
use crate::Result;

/// A client to GitHub's repository topics API.
///
/// Created with [`RepoHandler::topics`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-all-repository-topics)
pub struct RepoTopicsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoTopicsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Gets all repository topics using default pagination.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-all-repository-topics)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let topics = octocrab.repos("owner", "repo")
    ///     .topics()
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<RepoTopics> {
        self.list().send().await
    }

    /// Creates a [`ListRepoTopicsBuilder`] to configure pagination when fetching repository topics.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-all-repository-topics)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let topics = octocrab.repos("owner", "repo")
    ///     .topics()
    ///     .list()
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListRepoTopicsBuilder<'octo, 'r> {
        ListRepoTopicsBuilder::new(self.handler)
    }

    /// Replaces all repository topics.
    ///
    /// Pass an empty slice or collection to clear all topics from the repository.
    /// Note: Topic names will be saved as lowercase by GitHub.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#replace-all-repository-topics)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let topics = octocrab.repos("owner", "repo")
    ///     .topics()
    ///     .replace(["rust", "api"])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn replace(
        &self,
        topics: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<RepoTopics> {
        let route = format!("/{}/topics", self.handler.repo);
        let names: Vec<String> = topics.into_iter().map(Into::into).collect();
        let body = serde_json::json!({ "names": names });
        self.handler.crab.put(route, Some(&body)).await
    }
}

/// A builder pattern struct for listing repository topics with pagination.
///
/// Created by [`RepoTopicsHandler::list`].
#[derive(serde::Serialize)]
pub struct ListRepoTopicsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
}

impl<'octo, 'r> ListRepoTopicsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            page: None,
            per_page: None,
        }
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<RepoTopics> {
        let route = format!("/{}/topics", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
