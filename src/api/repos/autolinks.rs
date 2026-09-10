use super::RepoHandler;
use crate::models::{repos::Autolink, AutolinkId};
use crate::Result;

/// A client to GitHub's repository autolinks API.
///
/// Created with [`RepoHandler::autolinks`].
pub struct RepoAutolinksHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoAutolinksHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Gets all autolink references configured for a repository.
    ///
    /// Information about autolinks is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/autolinks#get-all-autolinks-of-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let autolinks = octocrab.repos("owner", "repo")
    ///     .autolinks()
    ///     .list()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Autolink>> {
        let route = format!("/{}/autolinks", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Gets a single autolink reference by its ID.
    ///
    /// Information about autolinks is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/autolinks#get-an-autolink-reference-of-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let autolink = octocrab.repos("owner", "repo")
    ///     .autolinks()
    ///     .get(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, autolink_id: impl Into<AutolinkId>) -> Result<Autolink> {
        let autolink_id = autolink_id.into();
        let route = format!("/{}/autolinks/{autolink_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a [`CreateAutolinkBuilder`] to configure a new autolink reference for a repository.
    ///
    /// Users with admin access to the repository can create an autolink.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/autolinks#create-an-autolink-reference-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let autolink = octocrab.repos("owner", "repo")
    ///     .autolinks()
    ///     .create("TICKET-", "https://example.com/TICKET?query=<num>")
    ///     .is_alphanumeric(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        key_prefix: impl Into<String>,
        url_template: impl Into<String>,
    ) -> CreateAutolinkBuilder<'octo, 'r> {
        CreateAutolinkBuilder::new(self.handler, key_prefix.into(), url_template.into())
    }

    /// Deletes an autolink reference from a repository.
    ///
    /// Information about autolinks is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/autolinks#delete-an-autolink-reference-from-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.repos("owner", "repo")
    ///     .autolinks()
    ///     .delete(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, autolink_id: impl Into<AutolinkId>) -> Result<()> {
        let autolink_id = autolink_id.into();
        let route = format!("/{}/autolinks/{autolink_id}", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for creating an autolink reference for a repository.
///
/// Created by [`RepoAutolinksHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateAutolinkBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    key_prefix: String,
    url_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_alphanumeric: Option<bool>,
}

impl<'octo, 'r> CreateAutolinkBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        key_prefix: String,
        url_template: String,
    ) -> Self {
        Self {
            handler,
            key_prefix,
            url_template,
            is_alphanumeric: None,
        }
    }

    /// Whether this autolink reference matches alphanumeric characters.
    /// If true, `<num>` matches `A-Z` (case insensitive), `0-9`, and `-`.
    /// If false, this autolink reference only matches numeric characters.
    pub fn is_alphanumeric(mut self, is_alphanumeric: impl Into<bool>) -> Self {
        self.is_alphanumeric = Some(is_alphanumeric.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Autolink> {
        let route = format!("/{}/autolinks", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}
