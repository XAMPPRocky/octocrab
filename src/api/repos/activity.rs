//! GitHub Repository Activity API.
//!
//! See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-repository-activities)

use super::RepoHandler;
use crate::{
    models::repos::{Activity, ActivityType},
    params::{repos::ActivityTimePeriod, Direction},
    Page, Result,
};

/// A client to GitHub's repository activity API.
///
/// Created with [`RepoHandler::activity`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-repository-activities)
pub struct RepoActivityHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoActivityHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`ListActivitiesBuilder`] to list activity in the repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let activities = octocrab
    ///     .repos("owner", "repo")
    ///     .activity()
    ///     .list()
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListActivitiesBuilder<'octo, 'r> {
        ListActivitiesBuilder::new(self.handler)
    }
}

/// A builder pattern struct for listing repository activity.
///
/// Created by [`RepoActivityHandler::list`] or [`RepoHandler::list_activities`].
#[derive(serde::Serialize)]
pub struct ListActivitiesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_period: Option<ActivityTimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<ActivityType>,
}

impl<'octo, 'r> ListActivitiesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            direction: None,
            per_page: None,
            before: None,
            after: None,
            r#ref: None,
            actor: None,
            time_period: None,
            activity_type: None,
        }
    }

    /// The direction to sort the results by: `asc` or `desc`. Default: `desc`.
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100). Default: 30.
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Cursor used for pagination; fetches results before this cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Cursor used for pagination; fetches results after this cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// The Git reference for the activities you want to list (e.g. `main` or `refs/heads/main`).
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// An alias for [`Self::ref`].
    pub fn git_ref(self, r#ref: impl Into<String>) -> Self {
        self.r#ref(r#ref)
    }

    /// The GitHub username to use to filter by the actor who performed the activity.
    pub fn actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }

    /// The time period to filter by: `day`, `week`, `month`, `quarter`, or `year`.
    pub fn time_period(mut self, time_period: impl Into<ActivityTimePeriod>) -> Self {
        self.time_period = Some(time_period.into());
        self
    }

    /// The activity type to filter by (e.g., `push`, `force_push`, `branch_creation`, etc.).
    pub fn activity_type(mut self, activity_type: impl Into<ActivityType>) -> Self {
        self.activity_type = Some(activity_type.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<Activity>> {
        let route = format!("/{}/activity", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
