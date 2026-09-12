use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Collected};

use crate::models::{CheckSuiteId, JobId, RunId};
use crate::{models, Octocrab, Page, Result};

pub struct WorkflowsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

/// Handler for GitHub's workflows API for actions.
///
/// Created with [`Octocrab::workflows`].
impl<'octo> WorkflowsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    async fn follow_location_to_data(
        &self,
        response: http::Response<BoxBody<Bytes, crate::Error>>,
    ) -> crate::Result<bytes::Bytes> {
        let data_response = self.crab.follow_location_to_data(response).await?;
        let body = data_response.into_body();
        body.collect().await.map(Collected::to_bytes)
    }

    /// List workflow definitions in the repository.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    ///
    /// let issue = octocrab.workflows("owner", "repo")
    ///     .list()
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListWorkflowsBuilder<'_, '_> {
        ListWorkflowsBuilder::new(self)
    }

    pub async fn get(&self, run_id: RunId) -> Result<models::workflows::Run> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// List runs in the specified workflow.
    /// workflow_file_or_id can be either file name or numeric expression.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    ///
    /// let issue = octocrab.workflows("owner", "repo")
    ///     .list_runs("ci.yml")
    ///     // Optional Parameters
    ///     .actor("octocat")
    ///     .branch("master")
    ///     .event("push")
    ///     .status("success")
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_runs(&self, workflow_file_or_id: impl Into<String>) -> ListRunsBuilder<'_, '_> {
        ListRunsBuilder::new(
            self,
            ListRunsRequestType::ByWorkflow(workflow_file_or_id.into()),
        )
    }

    /// List runs for the specified owner and repository.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    ///
    /// let runs = octocrab.workflows("owner", "repo")
    ///     .list_all_runs()
    ///     // Optional Parameters
    ///     .actor("octocat")
    ///     .branch("master")
    ///     .event("pull_request")
    ///     .status("success")
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_all_runs(&self) -> ListRunsBuilder<'_, '_> {
        ListRunsBuilder::new(self, ListRunsRequestType::ByRepo)
    }

    /// List job results in the specified run.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params::workflows::Filter;
    ///
    /// let issue = octocrab.workflows("owner", "repo")
    ///     .list_jobs(1234u64.into())
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(1u8)
    ///     .filter(Filter::All)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_jobs(&self, run_id: RunId) -> ListJobsBuilder<'_, '_> {
        ListJobsBuilder::new(self, run_id)
    }

    /// Gets a specific workflow in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflows?apiVersion=2022-11-28#get-a-workflow)
    pub async fn get_workflow(
        &self,
        workflow_id: impl Into<String>,
    ) -> Result<models::workflows::WorkFlow> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}",
            owner = self.owner,
            repo = self.repo,
            workflow_id = workflow_id.into(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Enables a workflow and sets the state of the workflow to active.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflows?apiVersion=2022-11-28#enable-a-workflow)
    pub async fn enable_workflow(&self, workflow_id: impl Into<String>) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable",
            owner = self.owner,
            repo = self.repo,
            workflow_id = workflow_id.into(),
        );
        crate::map_github_error(self.crab._put(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Disables a workflow and sets the state of the workflow to disabled_manually.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflows?apiVersion=2022-11-28#disable-a-workflow)
    pub async fn disable_workflow(&self, workflow_id: impl Into<String>) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable",
            owner = self.owner,
            repo = self.repo,
            workflow_id = workflow_id.into(),
        );
        crate::map_github_error(self.crab._put(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Gets workflow usage (billable minutes).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflows?apiVersion=2022-11-28#get-workflow-usage)
    pub async fn get_timing(
        &self,
        workflow_id: impl Into<String>,
    ) -> Result<models::workflows::WorkflowUsage> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing",
            owner = self.owner,
            repo = self.repo,
            workflow_id = workflow_id.into(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Cancels a workflow run and also terminates all of its running jobs.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#force-cancel-a-workflow-run)
    pub async fn force_cancel_run(&self, run_id: RunId) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/force-cancel",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        crate::map_github_error(self.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Re-runs all of the jobs in a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#re-run-a-workflow)
    pub async fn rerun_run(&self, run_id: RunId) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/rerun",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        crate::map_github_error(self.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Re-runs all of the failed jobs and their dependent jobs in a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#re-run-failed-jobs-from-a-workflow-run)
    pub async fn rerun_failed_jobs(&self, run_id: RunId) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        crate::map_github_error(self.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Gets the number of billable minutes and execution time for a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#get-workflow-run-usage)
    pub async fn get_run_timing(
        &self,
        run_id: RunId,
    ) -> Result<models::workflows::WorkflowRunUsage> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/timing",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get the review history for a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#get-the-review-history-for-a-workflow-run)
    pub async fn get_run_approvals(
        &self,
        run_id: RunId,
    ) -> Result<Vec<models::workflows::WorkflowRunApproval>> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/approvals",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Approves a workflow run for a fork pull request.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#approve-a-workflow-run-for-a-fork-pull-request)
    pub async fn approve_run(&self, run_id: RunId) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/approve",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        crate::map_github_error(self.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Get pending deployments for a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#get-pending-deployments-for-a-workflow-run)
    pub async fn get_run_pending_deployments(
        &self,
        run_id: RunId,
    ) -> Result<Vec<models::workflows::PendingDeployment>> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Review custom deployment protection rules for a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#review-custom-deployment-protection-rules-for-a-workflow-run)
    pub async fn review_custom_deployment_protection_rule(
        &self,
        run_id: RunId,
        environment_name: impl AsRef<str>,
        state: models::workflows::ReviewDeploymentState,
        comment: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/deployment_protection_rule",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );
        let body = serde_json::json!({
            "environment_name": environment_name.as_ref(),
            "state": state,
            "comment": comment.as_ref(),
        });
        crate::map_github_error(self.crab._post(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Gets a specific workflow run attempt.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#get-a-workflow-run-attempt)
    pub async fn get_attempt(
        &self,
        run_id: RunId,
        attempt_number: u32,
    ) -> Result<models::workflows::Run> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
            attempt_number = attempt_number,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Lists jobs for a workflow run attempt.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#list-jobs-for-a-workflow-run-attempt)
    pub fn list_attempt_jobs(&self, run_id: RunId, attempt_number: u32) -> ListJobsBuilder<'_, '_> {
        let mut builder = ListJobsBuilder::new(self, run_id);
        builder.attempt_number = Some(attempt_number);
        builder
    }

    /// Downloads and returns the raw data representing a zip of the logs from a workflow run attempt.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#download-workflow-run-attempt-logs)
    pub async fn download_attempt_logs(
        &self,
        run_id: RunId,
        attempt_number: u32,
    ) -> Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/logs",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
            attempt_number = attempt_number,
        );
        self.follow_location_to_data(self.crab._get(route).await?)
            .await
    }

    /// Gets a specific job in a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-jobs?apiVersion=2022-11-28#get-a-job-for-a-workflow-run)
    pub async fn get_job(&self, job_id: JobId) -> Result<models::workflows::Job> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/jobs/{job_id}",
            owner = self.owner,
            repo = self.repo,
            job_id = job_id,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Downloads and returns the raw text log for a workflow run job.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-jobs?apiVersion=2022-11-28#download-job-logs-for-a-workflow-run)
    pub async fn download_job_logs(&self, job_id: JobId) -> Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/jobs/{job_id}/logs",
            owner = self.owner,
            repo = self.repo,
            job_id = job_id,
        );
        self.follow_location_to_data(self.crab._get(route).await?)
            .await
    }

    /// Re-runs a job and its dependent jobs in a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/workflow-jobs?apiVersion=2022-11-28#re-run-a-job-from-a-workflow-run)
    pub async fn rerun_job(&self, job_id: JobId) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/jobs/{job_id}/rerun",
            owner = self.owner,
            repo = self.repo,
            job_id = job_id,
        );
        crate::map_github_error(self.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }
}

#[derive(serde::Serialize)]
pub struct ListWorkflowsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListWorkflowsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>) -> Self {
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::WorkFlow>> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// The type of list workflow runs request.
pub(crate) enum ListRunsRequestType {
    ByRepo,
    ByWorkflow(String),
}

#[derive(serde::Serialize)]
pub struct ListRunsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip)]
    r#type: ListRunsRequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_pull_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_suite_id: Option<CheckSuiteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_sha: Option<String>,
}

impl<'octo, 'b> ListRunsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>, r#type: ListRunsRequestType) -> Self {
        Self {
            handler,
            r#type,
            actor: None,
            branch: None,
            event: None,
            status: None,
            per_page: None,
            page: None,
            exclude_pull_requests: None,
            check_suite_id: None,
            head_sha: None,
        }
    }

    /// Someone who runs workflows. Use the login to specify a user.
    pub fn actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }

    /// A branch associated with workflows. Use the name of the branch of the push.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// An event associated with workflows. Can be e.g. push, pull_request, issue,
    /// ... and many variations. See official "Events that trigger workflows." doc.
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// A status associated with workflows.
    /// status or conclusion can be specified. e.g. success, in_progress, waiting...
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
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

    /// Whether to exclude the pull requests or not.
    pub fn exclude_pull_requests(mut self, exclude_pull_requests: impl Into<bool>) -> Self {
        self.exclude_pull_requests = Some(exclude_pull_requests.into());
        self
    }

    /// Returns workflow runs with the check_suite_id that you specify
    pub fn check_suite_id(mut self, check_suite_id: impl Into<CheckSuiteId>) -> Self {
        self.check_suite_id = Some(check_suite_id.into());
        self
    }

    /// Only returns workflow runs that are associated with the specified head_sha.
    pub fn head_sha(mut self, head_sha: impl Into<String>) -> Self {
        self.head_sha = Some(head_sha.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::Run>> {
        let route = match self.r#type {
            ListRunsRequestType::ByRepo => format!(
                "/repos/{owner}/{repo}/actions/runs",
                owner = self.handler.owner,
                repo = self.handler.repo
            ),
            ListRunsRequestType::ByWorkflow(ref workflow_id) => format!(
                "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs",
                owner = self.handler.owner,
                repo = self.handler.repo,
                workflow_id = workflow_id
            ),
        };
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListJobsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip)]
    run_id: RunId,
    #[serde(skip)]
    attempt_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<crate::params::workflows::Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListJobsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>, run_id: RunId) -> Self {
        Self {
            handler,
            run_id,
            attempt_number: None,
            per_page: None,
            page: None,
            filter: None,
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

    /// Filters jobs by their completed_at timestamp. Choose latest or all.
    pub fn filter(mut self, filter: impl Into<crate::params::workflows::Filter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::Job>> {
        let route = match self.attempt_number {
            Some(attempt_number) => format!(
                "/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs",
                owner = self.handler.owner,
                repo = self.handler.repo,
                run_id = self.run_id,
                attempt_number = attempt_number,
            ),
            None => format!(
                "/repos/{owner}/{repo}/actions/runs/{run_id}/jobs",
                owner = self.handler.owner,
                repo = self.handler.repo,
                run_id = self.run_id,
            ),
        };
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        use crate::params::workflows::Filter;

        let octocrab = crate::Octocrab::default();
        let handler = octocrab.workflows("rust-lang", "rust");
        let list_jobs = handler
            .list_jobs(1234u64.into())
            .filter(Filter::All)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list_jobs).unwrap(),
            serde_json::json!({
                "filter": "all",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
