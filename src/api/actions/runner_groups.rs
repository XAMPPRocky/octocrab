use crate::models::actions::{HostedRunnersResponse, RunnerGroup, RunnerGroupsResponse};
use crate::models::RunnerGroupId;
use crate::{Octocrab, Result};
use serde::Serialize;

/// Handler for GitHub Actions Runner Groups for an organization.
pub struct OrgRunnerGroupsHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> OrgRunnerGroupsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String) -> Self {
        Self { crab, org }
    }

    /// List self-hosted runner groups for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#list-self-hosted-runner-groups-for-an-organization)
    pub async fn list(&self) -> Result<RunnerGroupsResponse> {
        let route = format!("/orgs/{}/actions/runner-groups", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Create a self-hosted runner group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#create-a-self-hosted-runner-group-for-an-organization)
    pub async fn create(&self, body: &impl Serialize) -> Result<RunnerGroup> {
        let route = format!("/orgs/{}/actions/runner-groups", self.org);
        self.crab.post(route, Some(body)).await
    }

    /// Get a self-hosted runner group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#get-a-self-hosted-runner-group-for-an-organization)
    pub async fn get(&self, runner_group_id: impl Into<RunnerGroupId>) -> Result<RunnerGroup> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}",
            self.org,
            runner_group_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Update a self-hosted runner group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#update-a-self-hosted-runner-group-for-an-organization)
    pub async fn update(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        body: &impl Serialize,
    ) -> Result<RunnerGroup> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}",
            self.org,
            runner_group_id.into()
        );
        self.crab.patch(route, Some(body)).await
    }

    /// Delete a self-hosted runner group from an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#delete-a-self-hosted-runner-group-from-an-organization)
    pub async fn delete(&self, runner_group_id: impl Into<RunnerGroupId>) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}",
            self.org,
            runner_group_id.into()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// List GitHub-hosted runners in a group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#list-github-hosted-runners-in-a-group-for-an-organization)
    pub async fn list_hosted_runners(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
    ) -> Result<HostedRunnersResponse> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/hosted-runners",
            self.org,
            runner_group_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// List repository access to a self-hosted runner group in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#list-repository-access-to-a-self-hosted-runner-group-in-an-organization)
    pub async fn list_repositories(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories",
            self.org,
            runner_group_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set repository access for a self-hosted runner group in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#set-repository-access-for-a-self-hosted-runner-group-in-an-organization)
    pub async fn set_repositories(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories",
            self.org,
            runner_group_id.into()
        );
        let response = self.crab._put(route, Some(body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Add repository access to a self-hosted runner group in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#add-repository-access-to-a-self-hosted-runner-group-in-an-organization)
    pub async fn add_repository(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        repository_id: u64,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories/{}",
            self.org,
            runner_group_id.into(),
            repository_id
        );
        let response = self.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Remove repository access to a self-hosted runner group in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#remove-repository-access-to-a-self-hosted-runner-group-in-an-organization)
    pub async fn remove_repository(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        repository_id: u64,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories/{}",
            self.org,
            runner_group_id.into(),
            repository_id
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// List self-hosted runners in a group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#list-self-hosted-runners-in-a-group-for-an-organization)
    pub async fn list_runners(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/runners",
            self.org,
            runner_group_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set self-hosted runners in a group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#set-self-hosted-runners-in-a-group-for-an-organization)
    pub async fn set_runners(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/runners",
            self.org,
            runner_group_id.into()
        );
        let response = self.crab._put(route, Some(body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Add a self-hosted runner to a group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#add-a-self-hosted-runner-to-a-group-for-an-organization)
    pub async fn add_runner(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        runner_id: u64,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/runners/{}",
            self.org,
            runner_group_id.into(),
            runner_id
        );
        let response = self.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Remove a self-hosted runner from a group for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runner-groups?apiVersion=2022-11-28#remove-a-self-hosted-runner-from-a-group-for-an-organization)
    pub async fn remove_runner(
        &self,
        runner_group_id: impl Into<RunnerGroupId>,
        runner_id: u64,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/runner-groups/{}/runners/{}",
            self.org,
            runner_group_id.into(),
            runner_id
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
