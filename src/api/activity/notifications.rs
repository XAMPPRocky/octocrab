//! Github Notifications API

use std::borrow::Cow;

use crate::models::activity::Notification;
use crate::models::activity::ThreadSubscription;
use crate::models::{NotificationId, ThreadId};
use crate::Octocrab;
use crate::Page;

type DateTime = chrono::DateTime<chrono::Utc>;

/// Handler for GitHub's notifications API.
///
/// Created with [`ActivityHandler::notifications`].
/// **Note:** All of these methods require authentication using
/// your GitHub Access Token with the right privileges.
///
/// [`ActivityHandler::notifications`]: ../struct.ActivityHandler.html#method.notifications
#[derive(octocrab_derive::Builder)]
pub struct NotificationsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> NotificationsHandler<'octo> {
    /// Gets a notification by their id.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let thread = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .get(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: NotificationId) -> crate::Result<Notification> {
        let url = format!("notifications/threads/{}", id);
        self.crab.get(url, None::<&()>).await
    }

    /// Marks a single thread as read.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .mark_as_read(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_as_read(&self, id: NotificationId) -> crate::Result<()> {
        let url = format!("notifications/threads/{}", id);
        let url = self.crab.absolute_url(url)?;

        let response = self.crab._patch(url, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Marks all notifications in a repository as read.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .mark_repo_as_read("XAMPPRocky", "octocrab", None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_repo_as_read(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        last_read_at: impl Into<Option<DateTime>>,
    ) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        struct Inner {
            last_read_at: DateTime,
        }

        let body = last_read_at
            .into()
            .map(|last_read_at| Inner { last_read_at });

        let url = format!("repos/{}/{}/notifications", owner.as_ref(), repo.as_ref());
        let url = self.crab.absolute_url(url)?;

        let response = self.crab._put(url, body.as_ref()).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Marks all notifications as read.
    ///
    /// If you provide a `last_read_at` parameter,
    /// anything updated since this time will not be marked as read.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .mark_all_as_read(None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_all_as_read(
        &self,
        last_read_at: impl Into<Option<DateTime>>,
    ) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        struct Inner {
            last_read_at: DateTime,
        }

        let body = last_read_at
            .into()
            .map(|last_read_at| Inner { last_read_at });
        let url = self.crab.absolute_url("notifications")?;

        let response = self.crab._put(url, body.as_ref()).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// This checks to see if the current user is subscribed to a thread.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let subscription = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .get_thread_subscription(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_thread_subscription(
        &self,
        thread: ThreadId,
    ) -> crate::Result<ThreadSubscription> {
        let url = format!("notifications/threads/{}/subscription", thread);

        self.crab.get(url, None::<&()>).await
    }

    /// Ignore or unignore a thread subscription, that is enabled by watching a repository.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let subscription = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .set_thread_subscription(123u64.into(), true)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_thread_subscription(
        &self,
        thread: ThreadId,
        ignored: bool,
    ) -> crate::Result<ThreadSubscription> {
        #[derive(serde::Serialize)]
        struct Inner {
            ignored: bool,
        }

        let url = format!("notifications/threads/{}/subscription", thread);
        let body = Inner { ignored };

        self.crab.get(url, Some(&body)).await
    }

    /// Mutes the whole thread conversation until you comment or get mentioned.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .delete_thread_subscription(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_thread_subscription(&self, thread: ThreadId) -> crate::Result<()> {
        let url = self.crab.absolute_url(format!(
            "notifications/threads/{}/subscription",
            thread
        ))?;

        let response = self.crab._delete(url, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List all notifications for the current user, that are in a given repository.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let notifications = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .list_for_repo("XAMPPRocky", "octocrab")
    ///     // Also show notifications that are marked as read.
    ///     .all(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_for_repo(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> ListNotificationsBuilder<'octo, '_> {
        let url = format!("repos/{}/{}/notifications", owner.as_ref(), repo.as_ref());
        ListNotificationsBuilder::new(self.crab, Cow::from(url))
    }

    /// List all notifications for the current user.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let notifications = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListNotificationsBuilder<'octo, '_> {
        ListNotificationsBuilder::new(self.crab, Cow::from("notifications"))
    }
}

/// A builder pattern struct for listing pull requests.
///
/// Created by [`NotificationsHandler::list`].
///
/// [`NotificationsHandler::list`]: ./struct.NotificationsHandler.html#method.list
#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListNotificationsBuilder<'octo, 'url> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    url: Cow<'url, str>,
    /// If set, show notifications marked as read.
    all: Option<bool>,
    /// If set, only shows notifications in which the user is directly participating or mentioned.
    participating: Option<bool>,
    /// Only show notifications updated after the given time.
    since: Option<chrono::DateTime<chrono::Utc>>,
    /// Only show notifications updated before the given time.
    before: Option<chrono::DateTime<chrono::Utc>>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u8>,
}

impl<'octo, 'url> ListNotificationsBuilder<'octo, 'url> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Notification>> {
        self.crab.get(&self.url, Some(&self)).await
    }
}
