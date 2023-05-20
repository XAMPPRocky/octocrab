//! The `users` API: `/user`, `/users`
//!
//! Endpoints under this deal with querying public, and private information
//! about specific users.
//!
//! [Official Documentation][docs]
//!
//! [docs]: https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28

use chrono::{DateTime, Utc};

use crate::{models::gists::Gist, Octocrab, Page};

/// Handler for GitHub's users API.
///
/// Created with [`Octocrab::users`]
pub struct UsersHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> UsersHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// List gists for the given username, allowing for pagination.
    ///
    /// See [GitHub API Documentation][docs] for details on `GET /users/{username}/gists`
    ///
    /// # Examples
    ///
    /// * Fetch 10 recent gists for the user with login "foouser":
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///     octocrab::instance()
    ///         .users()
    ///         .list_user_gists("foouser")
    ///         .page(1u32)
    ///         .per_page(10u8)
    ///         .send()
    ///         .await?;
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#list-gists-for-a-user
    pub fn list_user_gists(&self, username: impl AsRef<str>) -> ListGistsForUserBuilder<'octo> {
        ListGistsForUserBuilder::new(self.crab, username.as_ref().to_string())
    }
}

/// Handles query data for the `GET /users/{username}/gists` endpoint.
#[derive(Debug, serde::Serialize)]
pub struct ListGistsForUserBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip)]
    /// Username for which to retrieve gists
    username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListGistsForUserBuilder<'octo> {
    pub fn new(crab: &'octo Octocrab, username: String) -> Self {
        Self {
            crab,
            username,
            since: None,
            per_page: None,
            page: None,
        }
    }

    pub fn since(mut self, last_updated: DateTime<Utc>) -> Self {
        self.since = Some(last_updated);
        self
    }

    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    pub fn page(mut self, number: u32) -> Self {
        self.page = Some(number);
        self
    }

    pub async fn send(self) -> crate::Result<Page<Gist>> {
        self.crab
            .get(
                format!("/users/{username}/gists", username = self.username),
                Some(&self),
            )
            .await
    }
}
