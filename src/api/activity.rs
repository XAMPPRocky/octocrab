//! Github Activity API

use crate::Octocrab;

pub mod notifications;

/// Handler for GitHub's activity API.
///
/// Created with [`Octocrab::activity`].
///
/// [`Octocrab::activity`]: ../struct.Octocrab.html#method.activity
pub struct ActivityHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ActivityHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Creates a `NotificationsHandler` for the current authenticated user.
    pub fn notifications(&self) -> notifications::NotificationsHandler<'octo> {
        notifications::NotificationsHandler::new(self.crab)
    }
}
