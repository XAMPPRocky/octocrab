use super::*;
use crate::models::TagProtectionId;

/// A client to GitHub's repository tags API.
///
/// Created with [`RepoHandler::tags`].
pub struct RepoTagsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoTagsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List tags from a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-repository-tags)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let tags = octocrab.repos("owner", "repo")
    ///     .tags()
    ///     .list()
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListTagsBuilder<'octo, 'r> {
        ListTagsBuilder::new(self.handler)
    }

    /// Creates a `RepoTagProtectionHandler` for managing tag protection in the repository.
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    #[allow(deprecated)]
    pub fn protection(&self) -> RepoTagProtectionHandler<'octo, 'r> {
        RepoTagProtectionHandler::new(self.handler)
    }

    /// Gets the tag protection states of a repository.
    ///
    /// This information is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/tags?apiVersion=2022-11-28#list-tag-protection-states-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// # #[allow(deprecated)]
    /// let protections = octocrab.repos("owner", "repo")
    ///     .tags()
    ///     .list_protection()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    #[allow(deprecated)]
    pub async fn list_protection(&self) -> crate::Result<Vec<crate::models::repos::TagProtection>> {
        self.protection().list().await
    }

    /// Creates a tag protection state for a repository.
    ///
    /// This endpoint is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/tags?apiVersion=2022-11-28#create-a-tag-protection-state-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// # #[allow(deprecated)]
    /// let protection = octocrab.repos("owner", "repo")
    ///     .tags()
    ///     .create_protection("v1.*")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    #[allow(deprecated)]
    pub async fn create_protection(
        &self,
        pattern: impl Into<String>,
    ) -> crate::Result<crate::models::repos::TagProtection> {
        self.protection().create(pattern).await
    }

    /// Deletes a tag protection state for a repository.
    ///
    /// This endpoint is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/tags?apiVersion=2022-11-28#delete-a-tag-protection-state-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// # #[allow(deprecated)]
    /// octocrab.repos("owner", "repo")
    ///     .tags()
    ///     .delete_protection(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    #[allow(deprecated)]
    pub async fn delete_protection(
        &self,
        tag_protection_id: impl Into<TagProtectionId>,
    ) -> crate::Result<()> {
        self.protection().delete(tag_protection_id).await
    }
}

/// A client to GitHub's repository tag protection API.
///
/// Created with [`RepoTagsHandler::protection`] or [`RepoHandler::tag_protection`].
#[deprecated(note = "Tag protection is closing down in GitHub. Use repository rulesets instead.")]
pub struct RepoTagProtectionHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

#[allow(deprecated)]
impl<'octo, 'r> RepoTagProtectionHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Gets the tag protection states of a repository.
    ///
    /// This information is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/tags?apiVersion=2022-11-28#list-tag-protection-states-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// # #[allow(deprecated)]
    /// let protections = octocrab.repos("owner", "repo")
    ///     .tag_protection()
    ///     .list()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    pub async fn list(&self) -> crate::Result<Vec<crate::models::repos::TagProtection>> {
        let route = format!("/{}/tags/protection", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a tag protection state for a repository.
    ///
    /// This endpoint is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/tags?apiVersion=2022-11-28#create-a-tag-protection-state-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// # #[allow(deprecated)]
    /// let protection = octocrab.repos("owner", "repo")
    ///     .tag_protection()
    ///     .create("v1.*")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    pub async fn create(
        &self,
        pattern: impl Into<String>,
    ) -> crate::Result<crate::models::repos::TagProtection> {
        let route = format!("/{}/tags/protection", self.handler.repo);
        self.handler
            .crab
            .post(
                route,
                Some(&serde_json::json!({ "pattern": pattern.into() })),
            )
            .await
    }

    /// Deletes a tag protection state for a repository.
    ///
    /// This endpoint is only available to repository administrators.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/tags?apiVersion=2022-11-28#delete-a-tag-protection-state-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// # #[allow(deprecated)]
    /// octocrab.repos("owner", "repo")
    ///     .tag_protection()
    ///     .delete(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        note = "Tag protection is closing down in GitHub. Use repository rulesets instead."
    )]
    pub async fn delete(&self, tag_protection_id: impl Into<TagProtectionId>) -> crate::Result<()> {
        let tag_protection_id = tag_protection_id.into();
        let route = format!("/{}/tags/protection/{tag_protection_id}", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct ListTagsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListTagsBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::Tag>> {
        let route = format!("/{}/tags", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
