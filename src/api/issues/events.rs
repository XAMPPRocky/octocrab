use super::IssueHandler;
use crate::models::{IssueEvent, IssueEventId};
use crate::Result;

/// Handler for GitHub's repository issue events API.
///
/// Created with [`IssueHandler::events`].
pub struct IssueEventsHandler<'octo, 'r> {
    handler: &'r IssueHandler<'octo>,
}

impl<'octo, 'r> IssueEventsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List issue events for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/events?apiVersion=2022-11-28#list-issue-events-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let events = octocrab.issues("owner", "repo")
    ///     .events()
    ///     .list()
    ///     .per_page(100)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListIssueEventsBuilder<'octo, 'r> {
        ListIssueEventsBuilder::new(self.handler)
    }

    /// Get an issue event from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/events?apiVersion=2022-11-28#get-an-issue-event)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let event = octocrab.issues("owner", "repo")
    ///     .events()
    ///     .get(1u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, event_id: impl Into<IssueEventId>) -> Result<IssueEvent> {
        let route = format!(
            "/{repo}/issues/events/{event_id}",
            repo = self.handler.repo,
            event_id = event_id.into(),
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// List issue events for an issue in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/events?apiVersion=2022-11-28#list-issue-events-for-an-issue)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let events = octocrab.issues("owner", "repo")
    ///     .events()
    ///     .list_for_issue(1)
    ///     .per_page(100)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_for_issue(&self, issue_number: u64) -> ListEventsForIssueBuilder<'octo, 'r> {
        ListEventsForIssueBuilder::new(self.handler, issue_number)
    }
}

#[derive(serde::Serialize)]
pub struct ListIssueEventsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListIssueEventsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self {
            handler,
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

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<IssueEvent>> {
        let route = format!("/{repo}/issues/events", repo = self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListEventsForIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    issue_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListEventsForIssueBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, issue_number: u64) -> Self {
        Self {
            handler,
            issue_number,
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

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<IssueEvent>> {
        let route = format!(
            "/{repo}/issues/{issue_number}/events",
            repo = self.handler.repo,
            issue_number = self.issue_number,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
