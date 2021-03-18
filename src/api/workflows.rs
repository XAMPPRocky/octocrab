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
        Self {
            crab,
            owner: owner,
            repo: repo,
        }
    }

    /// List workflow definitions in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
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

    /// List runs in the specified workflow.
    /// workflow_file_or_id can be either file name or numeric expression.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
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
        ListRunsBuilder::new(self, workflow_file_or_id.into())
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
        let url = format!(
            "repos/{owner}/{repo}/actions/workflows",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListRunsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip)]
    workflow_id: String,
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
}

impl<'octo, 'b> ListRunsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>, workflow_id: String) -> Self {
        Self {
            handler,
            workflow_id: workflow_id,
            actor: None,
            branch: None,
            event: None,
            status: None,
            per_page: None,
            page: None,
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

    /// An event associated with workflows. Can be e.g. pusu, pull_request, issue,
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::Run>> {
        let url = format!(
            "repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs",
            owner = self.handler.owner,
            repo = self.handler.repo,
            workflow_id = self.workflow_id
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}
