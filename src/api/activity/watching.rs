//! GitHub Watching API

use serde::Serialize;

pub use crate::api::repos::{
    ListWatchersBuilder, RepoSubscriptionHandler, SetRepoSubscriptionBuilder,
};
use crate::models::{activity::RepositorySubscription, Repository};
use crate::{Octocrab, Page, Result};

pub type SetRepositorySubscriptionBuilder<'octo> = SetRepoSubscriptionBuilder<'octo>;

/// Handler for GitHub's watching API.
///
/// Created with [`ActivityHandler::watching`].
///
/// [`ActivityHandler::watching`]: ../struct.ActivityHandler.html#method.watching
pub struct WatchingHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> WatchingHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Lists the people watching the specified repository.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#list-watchers)
    pub fn list_watchers(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> ListWatchersBuilder<'octo> {
        ListWatchersBuilder::new(
            self.crab,
            format!("/repos/{}/{}/subscribers", owner.into(), repo.into()),
        )
    }

    /// Gets information about whether the authenticated user is subscribed to the repository.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#get-a-repository-subscription)
    pub async fn get_repository_subscription(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<RepositorySubscription> {
        let route = format!(
            "/repos/{owner}/{repo}/subscription",
            owner = owner.as_ref(),
            repo = repo.as_ref()
        );
        RepoSubscriptionHandler::new(self.crab, route).get().await
    }

    /// Sets a subscription to a repository.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#set-a-repository-subscription)
    pub fn set_repository_subscription(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> SetRepoSubscriptionBuilder<'octo> {
        let route = format!("/repos/{}/{}/subscription", owner.into(), repo.into());
        SetRepoSubscriptionBuilder::new(self.crab, route)
    }

    /// Deletes a repository subscription.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#delete-a-repository-subscription)
    pub async fn delete_repository_subscription(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/subscription",
            owner = owner.as_ref(),
            repo = repo.as_ref()
        );
        RepoSubscriptionHandler::new(self.crab, route).delete().await
    }

    /// Lists repositories the authenticated user is watching.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#list-repositories-watched-by-the-authenticated-user)
    pub fn list_watched_repos_for_authenticated_user(&self) -> ListUserSubscriptionsBuilder<'octo> {
        ListUserSubscriptionsBuilder::new(self.crab, "/user/subscriptions")
    }

    /// Lists repositories a user is watching.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#list-repositories-watched-by-a-user)
    pub fn list_watched_repos_for_user(
        &self,
        username: impl Into<String>,
    ) -> ListUserSubscriptionsBuilder<'octo> {
        ListUserSubscriptionsBuilder::new(
            self.crab,
            format!("/users/{username}/subscriptions", username = username.into()),
        )
    }
}

/// A builder pattern struct for listing watched repositories.
#[derive(Serialize)]
pub struct ListUserSubscriptionsBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListUserSubscriptionsBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, route: impl Into<String>) -> Self {
        Self {
            crab,
            route: route.into(),
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

    /// Sends the request.
    pub async fn send(self) -> Result<Page<Repository>> {
        self.crab.get(&self.route, Some(&self)).await
    }
}
