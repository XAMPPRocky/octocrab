//! Github Activity API

use crate::Octocrab;

pub mod notifications;

/// Handler for GitHub's activity API.
///
/// Created with [`Octocrab::activity`].
#[derive(octocrab_derive::Builder)]
pub struct ActivityHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ActivityHandler<'octo> {
    /// Creates a `NotificationsHandler` for the current authenticated user.
    pub fn notifications(&self) -> notifications::NotificationsHandler<'octo> {
        notifications::NotificationsHandler::new(self.crab)
    }
}
