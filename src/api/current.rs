//! Get data about the currently authenticated user.

use crate::{
    models::{self, Repository},
    Octocrab, Page, Result,
};
use chrono::{DateTime, Utc};

/// Handler for the current authenication API. **Note** All of the methods
/// provided below require at least some authenication such as personal token
/// in order to be used.
///
/// Created with [`Octocrab::current`].
pub struct CurrentAuthHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CurrentAuthHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Fetches information about the current user.
    pub async fn user(&self) -> Result<models::User> {
        self.crab.get("user", None::<&()>).await
    }

    /// Fetches information about the currently authenticated app.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///
    /// let app = octocrab
    ///     .current()
    ///     .app()
    ///     .await?;
    ///
    /// println!("{}", app.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn app(&self) -> Result<models::App> {
        self.crab.get("app", None::<&()>).await
    }

    /// List repositories starred by current authenticated user.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_repos_starred_by_authenticated_user()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user)
    pub fn list_repos_starred_by_authenticated_user(&self) -> ListStarredReposBuilder<'octo> {
        ListStarredReposBuilder::new(self.crab)
    }

    /// Lists repositories that the current authenticated user.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_repos_for_authenticated_user()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user)
    pub fn list_repos_for_authenticated_user(&self) -> ListReposForAuthenticatedUserBuilder<'octo> {
        ListReposForAuthenticatedUserBuilder::new(&self.crab)
    }
}

/// A builder pattern struct for listing starred repositories.
///
/// Created by [`CurrentAuthHandler::list_repos_starred_by_authenticated_user`].
///
/// [`CurrentAuthHandler::list_repos_starred_by_authenticated_user`]: ./struct.CurrentAuthHandler.html#method.list_repos_starred_by_authenticated_user
#[derive(serde::Serialize)]
pub struct ListStarredReposBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo> ListStarredReposBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// One of `asc` (ascending) or `desc` (descending).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Repository>> {
        self.crab.get("user/starred", Some(&self)).await
    }
}

/// A builder pattern struct for listing repositories for authenticated user.
///
/// Created by [`CurrentAuthHandler::list_repos_for_authenticated_user`].
///
/// [`CurrentAuthHandler::list_repos_for_authenticated_user`]: ./struct.CurrentAuthHandler.html#method.list_repos_for_authenticated_user
#[derive(serde::Serialize)]
pub struct ListReposForAuthenticatedUserBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    affiliation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime<Utc>>,
}

impl<'octo> ListReposForAuthenticatedUserBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            visibility: None,
            affiliation: None,
            r#type: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
            since: None,
            before: None,
        }
    }

    /// Can be one of `all`, `public`, or `private`. Note: For GitHub AE, can be one of `all`, `internal`, or `private`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    /// Comma-separated list of values. Can include:
    /// * `owner`: Repositories that are owned by the authenticated user.
    /// * `collaborator`: Repositories that the user has been added to as a collaborator.
    /// * `organization_member`: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn affiliation(mut self, affiliation: impl Into<String>) -> Self {
        self.affiliation = Some(affiliation.into());
        self
    }

    /// Can be one of `all`, `owner`, `public`, `private`, `member`.
    ///
    /// Note: For GitHub AE, can be one of `all`, `owner`, `internal`, `private`, `member`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.r#type = Some(type_.into());
        self
    }

    /// Can be one of `created`, `updated`, `pushed`, `full_name`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Can be one of `asc` or `desc`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Only show notifications updated after the given time.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn since(mut self, since: impl Into<DateTime<Utc>>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Only show notifications updated before the given time.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn before(mut self, before: impl Into<DateTime<Utc>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Repository>> {
        self.crab.get("user/repos", (&self).into()).await
    }
}
