//! Github Activity API

use crate::Octocrab;

pub mod events;
pub mod notifications;
pub mod starring;
pub mod watching;

/// Handler for GitHub's activity API.
///
/// Created with [`Octocrab::activity`].
pub struct ActivityHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ActivityHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Creates a [`notifications::NotificationsHandler`] for the current authenticated user.
    pub fn notifications(&self) -> notifications::NotificationsHandler<'octo> {
        notifications::NotificationsHandler::new(self.crab)
    }

    /// Creates a [`starring::StarringHandler`] for accessing GitHub's starring API.
    pub fn starring(&self) -> starring::StarringHandler<'octo> {
        starring::StarringHandler::new(self.crab)
    }

    /// Creates a [`watching::WatchingHandler`] for accessing GitHub's watching/subscription API.
    pub fn watching(&self) -> watching::WatchingHandler<'octo> {
        watching::WatchingHandler::new(self.crab)
    }

    /// Creates an [`events::ActivityEventsHandler`] for accessing GitHub's activity events API.
    pub fn events(&self) -> events::ActivityEventsHandler<'octo> {
        events::ActivityEventsHandler::new(self.crab)
    }

    /// Lists the feeds available to the authenticated user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/activity/feeds?apiVersion=2022-11-28#get-feeds)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let feeds = octocrab.activity().feeds().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn feeds(&self) -> crate::Result<crate::models::activity::Feeds> {
        self.crab.get("/feeds", None::<&()>).await
    }
}
