//! Github Notifications API

use crate::Octocrab;
use crate::Page;
use crate::models::activity::Notification;
use crate::models::activity::ThreadSubscription;

type DateTime = chrono::DateTime<chrono::Utc>;

/// Handler for GitHub's notifications API.
///
/// Created with [`ActivityHandler::notifications`].
///
/// [`ActivityHandler::notifications`]: ../struct.ActivityHandler.html#method.notifications
pub struct NotificationsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> NotificationsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Gets a notification by their id.
    pub async fn get(&self, id: impl Into<u64>) -> crate::Result<Notification> {
        let url = format!("/notifications/threads/{}", id.into());
        self.crab.get(url, None::<&()>).await
    }
    
    /// Marks a single thread as read.
    pub async fn mark_as_read(&self, id: impl Into<u64>) -> crate::Result<()> {
        let url = format!("/notifications/threads/{}", id.into());
        let url = self.crab.absolute_url(url)?;

        let response = self.crab._patch(url, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Marks all notifications as read.
    ///
    /// If you provide a `last_read_at` parameter,
    /// anything updated since this time will not be marked as read.
    pub async fn mark_all_as_read(&self, last_read_at: impl Into<Option<DateTime>>) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        struct Inner {
            last_read_at: DateTime,
        }

        let body = last_read_at.into().map(|last_read_at| Inner { last_read_at });
        let url = self.crab.absolute_url("/notifications")?;

        let response = self.crab._put(url, body.as_ref()).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// This checks to see if the current user is subscribed to a thread.
    pub async fn get_thread_subscription(&self, thread: impl Into<u64>) -> crate::Result<ThreadSubscription> {
        let url = format!("/notifications/threads/{}/subscription", thread.into());

        self.crab.get(url, None::<&()>).await
    }

    /// Ignore or unignore a thread subscription, that is enabled by watching a repository.
    pub async fn set_thread_subscription(&self, thread: impl Into<u64>, ignored: bool) -> crate::Result<ThreadSubscription> {
        #[derive(serde::Serialize)]
        struct Inner {
            ignored: bool,
        }

        let url = format!("/notifications/threads/{}/subscription", thread.into());
        let body = Inner { ignored };

        self.crab.get(url, Some(&body)).await
    }

    /// Mutes the whole thread conversation until you comment or get mentioned.
    pub async fn delete_thread_subscription(&self, thread: impl Into<u64>) -> crate::Result<()> {
        let url = self.crab.absolute_url(format!("/notifications/threads/{}/subscription", thread.into()))?;

        let response = self.crab._delete(url, body.as_ref()).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List all notifications for the current user.
    pub fn list(&self) -> ListNotificationsBuilder<'octo> {
        ListNotificationsBuilder::new(self.crab)
    }
}

/// A builder pattern struct for listing pull requests.
///
/// Created by [`NotificationsHandler::list`].
///
/// [`NotificationsHandler::list`]: ./struct.NotificationsHandler.html#method.list
#[derive(serde::Serialize)]
pub struct ListNotificationsBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    participating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo> ListNotificationsBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            all: None,
            participating: None,
            since: None,
            before: None,
            per_page: None,
            page: None,
        }
    }

    /// If set, show notifications marked as read.
    pub fn all(mut self, v: bool) -> Self {
        self.all = Some(v);
        self
    }

    /// If set, only shows notifications in which the user is directly participating or mentioned.
    pub fn participating(mut self, v: bool) -> Self {
        self.participating = Some(v);
        self
    }
    
    /// Only show notifications updated after the given time.
    pub fn since(mut self, since: chrono::DateTime<chrono::Utc>) -> Self {
        self.since = Some(since);
        self
    }

    /// Only show notifications updated before the given time.
    pub fn before(mut self, before: chrono::DateTime<chrono::Utc>) -> Self {
        self.before = Some(before);
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Notification>> {
        self.crab.get("/notifications", Some(&self)).await
    }
}
