//! GitHub Repository Subscription API

use serde::Serialize;

use crate::models::activity::RepositorySubscription;
use crate::{Octocrab, Result};

pub struct RepoSubscriptionHandler<'octo> {
    crab: &'octo Octocrab,
    route: String,
}

impl<'octo> RepoSubscriptionHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, route: impl Into<String>) -> Self {
        Self {
            crab,
            route: route.into(),
        }
    }

    /// Gets information about whether the authenticated user is subscribed to the repository.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#get-a-repository-subscription)
    pub async fn get(&self) -> Result<RepositorySubscription> {
        self.crab.get(&self.route, None::<&()>).await
    }

    /// Sets a subscription to the repository.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#set-a-repository-subscription)
    pub fn set(&self) -> SetRepoSubscriptionBuilder<'octo> {
        SetRepoSubscriptionBuilder::new(self.crab, &self.route)
    }

    /// Deletes a repository subscription.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#delete-a-repository-subscription)
    pub async fn delete(&self) -> Result<()> {
        let response = self.crab._delete(&self.route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct SetRepoSubscriptionBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignored: Option<bool>,
}

impl<'octo> SetRepoSubscriptionBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, route: impl Into<String>) -> Self {
        Self {
            crab,
            route: route.into(),
            subscribed: None,
            ignored: None,
        }
    }

    /// Determines if notifications should be received from this repository.
    pub fn subscribed(mut self, subscribed: bool) -> Self {
        self.subscribed = Some(subscribed);
        self
    }

    /// Determines if all notifications should be blocked from this repository.
    pub fn ignored(mut self, ignored: bool) -> Self {
        self.ignored = Some(ignored);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<RepositorySubscription> {
        self.crab.put(&self.route, Some(&self)).await
    }
}
