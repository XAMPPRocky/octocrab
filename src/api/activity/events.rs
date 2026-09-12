//! GitHub Activity Events API

use crate::api::events::EventsBuilder;
use crate::Octocrab;

/// Handler for GitHub's activity events API.
///
/// Created with [`ActivityHandler::events`].
///
/// [`ActivityHandler::events`]: ../struct.ActivityHandler.html#method.events
pub struct ActivityEventsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ActivityEventsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// List public events for a network of repositories.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-public-events-for-a-network-of-repositories)
    pub fn network_events(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> EventsBuilder<'octo> {
        let route = format!("/networks/{}/{}/events", owner.into(), repo.into());
        EventsBuilder::with_route(self.crab, route)
    }

    /// List events for the authenticated user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-events-for-the-authenticated-user)
    pub fn user_events(&self, username: impl Into<String>) -> EventsBuilder<'octo> {
        let route = format!("/users/{}/events", username.into());
        EventsBuilder::with_route(self.crab, route)
    }

    /// List public events for a user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-public-events-for-a-user)
    pub fn public_user_events(&self, username: impl Into<String>) -> EventsBuilder<'octo> {
        let route = format!("/users/{}/events/public", username.into());
        EventsBuilder::with_route(self.crab, route)
    }

    /// List events received by the authenticated user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-events-received-by-the-authenticated-user)
    pub fn received_events(&self, username: impl Into<String>) -> EventsBuilder<'octo> {
        let route = format!("/users/{}/received_events", username.into());
        EventsBuilder::with_route(self.crab, route)
    }

    /// List public events received by a user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-public-events-received-by-a-user)
    pub fn public_received_events(&self, username: impl Into<String>) -> EventsBuilder<'octo> {
        let route = format!("/users/{}/received_events/public", username.into());
        EventsBuilder::with_route(self.crab, route)
    }

    /// List organization events for the authenticated user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-organization-events-for-the-authenticated-user)
    pub fn user_org_events(
        &self,
        username: impl Into<String>,
        org: impl Into<String>,
    ) -> EventsBuilder<'octo> {
        let route = format!("/users/{}/events/orgs/{}", username.into(), org.into());
        EventsBuilder::with_route(self.crab, route)
    }
}
