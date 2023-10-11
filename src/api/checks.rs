use crate::models::{CheckRunId, CheckSuiteId};
use crate::params::checks::{CheckRunConclusion, CheckRunOutput, CheckRunStatus};
use crate::params::repos::Commitish;
use crate::{models, Octocrab, Result};
use chrono::{DateTime, Utc};

/// Handler for GitHub's Checks API.
///
/// Created with [`Octocrab::checks`].
pub struct ChecksHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

#[derive(serde::Serialize)]
pub struct CreateCheckRunBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ChecksHandler<'octo>,
    name: String,
    head_sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<CheckRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conclusion: Option<CheckRunConclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<CheckRunOutput>,
}

impl<'octo, 'r> CreateCheckRunBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ChecksHandler<'octo>, name: String, head_sha: String) -> Self {
        Self {
            handler,
            name,
            head_sha,
            details_url: None,
            external_id: None,
            status: None,
            conclusion: None,
            completed_at: None,
            output: None,
        }
    }

    /// The URL of the integrator's site that has the full details of the check.
    /// If the integrator does not provide this, then the homepage of the GitHub app is used.
    pub fn details_url(mut self, details_url: impl Into<String>) -> Self {
        self.details_url = Some(details_url.into());
        self
    }

    /// A reference for the run on the integrator's system.
    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    /// The current status.
    /// Can be one of `queued`, `in_progress`, or `completed`.
    pub fn status(mut self, status: CheckRunStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// The final conclusion of the check.
    pub fn conclusion(mut self, conclusion: CheckRunConclusion) -> Self {
        self.conclusion = Some(conclusion);
        self
    }

    /// The time that the check run completed.
    pub fn completed_at(mut self, completed_at: DateTime<Utc>) -> Self {
        self.completed_at = Some(completed_at);
        self
    }

    /// Check runs can accept a variety of data in the output object,
    /// including a title and summary and can optionally provide
    /// descriptive details about the run.
    pub fn output(mut self, output: CheckRunOutput) -> Self {
        self.output = Some(output);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::checks::CheckRun> {
        let route = format!(
            "/repos/{owner}/{repo}/check-runs",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct UpdateCheckRunBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ChecksHandler<'octo>,
    check_run_id: CheckRunId,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<CheckRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conclusion: Option<CheckRunConclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<CheckRunOutput>,
}

impl<'octo, 'r> UpdateCheckRunBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ChecksHandler<'octo>, check_run_id: CheckRunId) -> Self {
        Self {
            handler,
            check_run_id,
            name: None,
            details_url: None,
            external_id: None,
            started_at: None,
            status: None,
            conclusion: None,
            completed_at: None,
            output: None,
        }
    }

    /// The name of the check. For example, "code-coverage".
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// The URL of the integrator's site that has the full details of the check.
    /// If the integrator does not provide this, then the homepage of the GitHub app is used.
    pub fn details_url(mut self, details_url: impl Into<String>) -> Self {
        self.details_url = Some(details_url.into());
        self
    }

    /// A reference for the run on the integrator's system.
    pub fn external_url(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    /// The time that the check run began.
    pub fn started_at(mut self, started_at: DateTime<Utc>) -> Self {
        self.started_at = Some(started_at);
        self
    }

    /// The current status.
    /// Can be one of `queued`, `in_progress`, or `completed`.
    pub fn status(mut self, status: CheckRunStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// The final conclusion of the check.
    pub fn conclusion(mut self, conclusion: CheckRunConclusion) -> Self {
        self.conclusion = Some(conclusion);
        self
    }

    /// The time that the check run completed.
    pub fn completed_at(mut self, completed_at: DateTime<Utc>) -> Self {
        self.completed_at = Some(completed_at);
        self
    }

    /// Check runs can accept a variety of data in the output object,
    /// including a title and summary and can optionally provide
    /// descriptive details about the run.
    pub fn output(mut self, output: CheckRunOutput) -> Self {
        self.output = Some(output);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::checks::CheckRun> {
        let route = format!(
            "/repos/{owner}/{repo}/check-runs/{check_run_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            check_run_id = self.check_run_id
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListCheckRunsInCheckSuiteBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ChecksHandler<'octo>,
    check_suite_id: CheckSuiteId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCheckRunsInCheckSuiteBuilder<'octo, 'r> {
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

#[derive(serde::Serialize)]
pub struct ListCheckRunsForGitRefBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ChecksHandler<'octo>,
    #[serde(skip)]
    git_ref: Commitish,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCheckRunsForGitRefBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ChecksHandler<'octo>, git_ref: Commitish) -> Self {
        Self {
            handler,
            git_ref,
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
            "/repos/{owner}/{repo}/commits/{ref}/check-runs",
            owner = self.handler.owner,
            repo = self.handler.repo,
            ref = self.git_ref,
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
    ) -> ListCheckRunsInCheckSuiteBuilder<'_, '_> {
        ListCheckRunsInCheckSuiteBuilder::new(self, suite_id)
    }

    /// ```no_run
    /// # use octocrab::params::repos::Commitish;
    ///  async fn run() -> octocrab::Result<()> {
    ///    let check_runs = octocrab::instance()
    ///      .checks("owner", "repo")
    ///      .list_check_runs_for_git_ref(Commitish("ref".to_string()))
    ///      .send()
    ///      .await?;
    /// # Ok(())
    /// # }
    pub fn list_check_runs_for_git_ref(
        &self,
        git_ref: Commitish,
    ) -> ListCheckRunsForGitRefBuilder<'_, '_> {
        ListCheckRunsForGitRefBuilder::new(self, git_ref)
    }

    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let check_run = octocrab::instance()
    ///    .checks("owner", "repo")
    ///    .create_check_run("name", "head_sha")
    ///    .details_url("https://example.com")
    ///    .external_id("external_id")
    ///    .status(octocrab::params::checks::CheckRunStatus::InProgress)
    ///    .send()
    ///    .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_check_run(
        &self,
        name: impl Into<String>,
        head_sha: impl Into<String>,
    ) -> CreateCheckRunBuilder<'_, '_> {
        CreateCheckRunBuilder::new(self, name.into(), head_sha.into())
    }

    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let check_run = octocrab::instance()
    ///   .checks("owner", "repo")
    ///  .update_check_run(123456.into())
    /// .name("name")
    /// .details_url("https://example.com")
    /// .external_url("external_id")
    /// .status(octocrab::params::checks::CheckRunStatus::InProgress)
    /// .send()
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_check_run(&self, check_run_id: CheckRunId) -> UpdateCheckRunBuilder<'_, '_> {
        UpdateCheckRunBuilder::new(self, check_run_id)
    }
}
