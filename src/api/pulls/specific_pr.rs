use crate::models::repos::RepoCommit;
use crate::pulls::PullRequestHandler;
use crate::Page;

/// A builder pattern struct for working with a specific pull request data,
/// e.g. reviews, commits, comments, etc.
///
/// created by [`PullRequestHandler::pull_number`]
///
/// [`PullRequestHandler::pull_number`]: ./struct.PullRequestHandler.html#method.pull_number
#[derive(serde::Serialize)]
pub struct SpecificPullRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip)]
    pr_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> SpecificPullRequestBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            pr_number,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max: 100, default: 30).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch. (default: 1)
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    ///Lists a maximum of 250 commits for a pull request.
    /// To receive a complete commit list for pull requests with more than 250 commits,
    /// use the [List commits](https://docs.github.com/rest/commits/commits#list-commits) endpoint.
    pub async fn commits(&self) -> crate::Result<Page<RepoCommit>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr_number}/commits",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr_number = self.pr_number
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
