use crate::models::actions::{ConcurrencyGroupDetails, ConcurrencyGroupsResponse};
use crate::models::RunId;
use crate::{Octocrab, Result};

/// Handler for Actions Concurrency Groups for a repository.
pub struct RepoActionsConcurrencyGroupsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> RepoActionsConcurrencyGroupsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// List concurrency groups for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/concurrency-groups?apiVersion=2022-11-28#list-concurrency-groups-for-a-repository)
    pub async fn list(&self) -> Result<ConcurrencyGroupsResponse> {
        let route = format!(
            "/repos/{}/{}/actions/concurrency_groups",
            self.owner, self.repo
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get a concurrency group for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/concurrency-groups?apiVersion=2022-11-28#get-a-concurrency-group-for-a-repository)
    pub async fn get(
        &self,
        concurrency_group_name: impl AsRef<str>,
    ) -> Result<ConcurrencyGroupDetails> {
        let route = format!(
            "/repos/{}/{}/actions/concurrency_groups/{}",
            self.owner,
            self.repo,
            concurrency_group_name.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// List concurrency groups for a workflow run.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/concurrency-groups?apiVersion=2022-11-28#list-concurrency-groups-for-a-workflow-run)
    pub async fn list_for_run(
        &self,
        run_id: impl Into<RunId>,
    ) -> Result<ConcurrencyGroupsResponse> {
        let route = format!(
            "/repos/{}/{}/actions/runs/{}/concurrency_groups",
            self.owner,
            self.repo,
            run_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }
}
