use crate::{Octocrab, Page};

pub mod create;
pub mod list;

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
    pub fn list(&'octo self) -> list::ListPullRequestsBuilder {
        list::ListPullRequestsBuilder::new(self)
    }
}

