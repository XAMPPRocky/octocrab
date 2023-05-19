use crate::models::CheckSuiteId;
use crate::{models, Octocrab, Result};

/// Handler for GitHub's Checks API.
///
/// Created with [`Octocrab::checks`].
pub struct ChecksHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

#[derive(serde::Serialize)]
pub struct ListCheckRunsinCheckSuiteBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ChecksHandler<'octo>,
    check_suite_id: CheckSuiteId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCheckRunsinCheckSuiteBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ChecksHandler<'octo>, check_suite_id: CheckSuiteId) -> Self {
        Self {
            handler,
            check_suite_id,
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
    pub async fn send(self) -> Result<models::checks::ListCheckRuns> {
        let route = format!(
            "/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs",
            owner = self.handler.owner,
            repo = self.handler.repo,
            check_suite_id = self.check_suite_id,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

impl<'octo> ChecksHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let check_runs = octocrab::instance()
    ///     .checks("owner", "repo")
    ///     .list_check_runs_in_a_check_suite(123456.into())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_check_runs_in_a_check_suite(
        &self,
        suite_id: CheckSuiteId,
    ) -> ListCheckRunsinCheckSuiteBuilder<'_, '_> {
        ListCheckRunsinCheckSuiteBuilder::new(self, suite_id)
    }
}
