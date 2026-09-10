use super::RepoHandler;
use crate::models::{repos::DeployKey, KeyId};
use crate::{Page, Result};

/// A client to GitHub's repository deploy keys API.
///
/// Created with [`RepoHandler::keys`].
pub struct RepoKeysHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoKeysHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`ListKeysBuilder`] to list deploy keys for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deploy-keys/deploy-keys#list-deploy-keys)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let keys = octocrab.repos("owner", "repo")
    ///     .keys()
    ///     .list()
    ///     .per_page(50)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListKeysBuilder<'octo, 'r> {
        ListKeysBuilder::new(self.handler)
    }

    /// Gets a single deploy key by its ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deploy-keys/deploy-keys#get-a-deploy-key)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let key = octocrab.repos("owner", "repo")
    ///     .keys()
    ///     .get(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, key_id: impl Into<KeyId>) -> Result<DeployKey> {
        let key_id = key_id.into();
        let route = format!("/{}/keys/{key_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a [`CreateKeyBuilder`] to create a new deploy key for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deploy-keys/deploy-keys#create-a-deploy-key)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let key = octocrab.repos("owner", "repo")
    ///     .keys()
    ///     .create("my-deploy-key", "ssh-rsa AAA...")
    ///     .read_only(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        title: impl Into<String>,
        key: impl Into<String>,
    ) -> CreateKeyBuilder<'octo, 'r> {
        CreateKeyBuilder::new(self.handler, title.into(), key.into())
    }

    /// Deletes a deploy key by its ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deploy-keys/deploy-keys#delete-a-deploy-key)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.repos("owner", "repo")
    ///     .keys()
    ///     .delete(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, key_id: impl Into<KeyId>) -> Result<()> {
        let key_id = key_id.into();
        let route = format!("/{}/keys/{key_id}", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for listing deploy keys for a repository.
///
/// Created by [`RepoKeysHandler::list`].
#[derive(serde::Serialize)]
pub struct ListKeysBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListKeysBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
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
    pub async fn send(self) -> Result<Page<DeployKey>> {
        let route = format!("/{}/keys", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a deploy key for a repository.
///
/// Created by [`RepoKeysHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateKeyBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    title: String,
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
}

impl<'octo, 'r> CreateKeyBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, title: String, key: String) -> Self {
        Self {
            handler,
            title,
            key,
            read_only: None,
        }
    }

    /// If `true`, the key will only be able to read repository contents.
    /// Otherwise, the key will be able to read and write.
    pub fn read_only(mut self, read_only: impl Into<bool>) -> Self {
        self.read_only = Some(read_only.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<DeployKey> {
        let route = format!("/{}/keys", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}
