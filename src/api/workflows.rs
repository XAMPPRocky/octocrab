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
    pub fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
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
