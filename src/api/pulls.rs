use crate::{Octocrab, Page};

pub mod create;

/// What to sort results by. Can be either `created`, `updated`, `popularity`
/// (comment count) or `long-running` (age, filtering by pulls updated in the
/// last month).
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestSorting {
    Created,
    Updated,
    Popularity,
    LongRunning,
}

/// What to sort results by. Can be either `created`, `updated`, `popularity`
/// (comment count) or `long-running` (age, filtering by pulls updated in the
/// last month).
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestDirection {
    Ascending,
    Descending,
}

pub struct PullRequestHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> PullRequestHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// Checks if a given pull request has been merged.
    pub async fn is_merged(&'octo self, pr: u64) -> crate::Result<bool> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/merge",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );
        let response = self
            .crab
            ._get(self.crab.absolute_url(route)?, None::<&()>)
            .await?;

        Ok(response.status() == 204)
    }

    /// Get's a given pull request with by its `pr` number.
    pub async fn get(&'octo self, pr: u64) -> crate::Result<crate::models::PullRequest> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Get's a given pull request with by its `pr` number.
    pub fn create(
        &'octo self,
        title: impl Into<String>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> create::CreatePullRequestBuilder<'octo, '_> {
        create::CreatePullRequestBuilder::new(self, title, head, base)
    }

    /// Creates a new `ListPullRequestsBuilder` that can be configured to filter
    /// listing pulling requests.
    pub fn list(&'octo self) -> ListPullRequestsBuilder {
        ListPullRequestsBuilder::new(self)
    }
}

#[derive(serde::Serialize)]
pub struct ListPullRequestsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    state: Option<super::StateParameter>,
    head: Option<String>,
    base: Option<String>,
    sort: Option<PullRequestSorting>,
    direction: Option<PullRequestDirection>,
    per_page: Option<u8>,
    page: Option<usize>,
}

impl<'octo, 'b> ListPullRequestsBuilder<'octo, 'b> {
    fn new(handler: &'b PullRequestHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            head: None,
            base: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter pull requests by `state`.
    pub fn state(mut self, state: super::StateParameter) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter pull requests by head user or head organization and branch name
    /// in the format of `user:ref-name` or `organization:ref-name`. For
    /// example: `github:new-script-format` or `octocrab:test-branch`.
    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    /// Filter pulls by base branch name. Example: `gh-pages`.
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    /// What to sort results by. Can be either `created`, `updated`,
    /// `popularity` (comment count) or `long-running` (age, filtering by pulls
    /// updated in the last month).
    pub fn sort(mut self, sort: impl Into<PullRequestSorting>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<PullRequestDirection>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<usize>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::PullRequest>> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize_list_pull_request() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .head("master")
            .base("branch")
            .sort(crate::pulls::PullRequestSorting::Popularity)
            .direction(crate::pulls::PullRequestDirection::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "head": "master",
                "base": "branch",
                "sort": "popularity",
                "direction": "ascending",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
