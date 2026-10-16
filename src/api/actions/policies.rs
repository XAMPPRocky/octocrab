use crate::models::actions::{ActionsPoliciesResponse, ActionsPolicy};
use crate::{Octocrab, Result};
use serde::Serialize;

/// Handler for Actions Policies for an organization.
pub struct OrgActionsPoliciesHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> OrgActionsPoliciesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String) -> Self {
        Self { crab, org }
    }

    /// List organization Actions policies.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#list-organization-actions-policies)
    pub async fn list(&self) -> Result<ActionsPoliciesResponse> {
        let route = format!("/orgs/{}/actions/policies", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Create an organization Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#create-an-organization-actions-policy)
    pub async fn create(&self, body: &impl Serialize) -> Result<ActionsPolicy> {
        let route = format!("/orgs/{}/actions/policies", self.org);
        self.crab.post(route, Some(body)).await
    }

    /// Get an organization Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#get-an-organization-actions-policy)
    pub async fn get(&self, policy_id: u64) -> Result<ActionsPolicy> {
        let route = format!("/orgs/{}/actions/policies/{}", self.org, policy_id);
        self.crab.get(route, None::<&()>).await
    }

    /// Update an organization Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#update-an-organization-actions-policy)
    pub async fn update(&self, policy_id: u64, body: &impl Serialize) -> Result<ActionsPolicy> {
        let route = format!("/orgs/{}/actions/policies/{}", self.org, policy_id);
        self.crab.put(route, Some(body)).await
    }

    /// Delete an organization Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#delete-an-organization-actions-policy)
    pub async fn delete(&self, policy_id: u64) -> Result<()> {
        let route = format!("/orgs/{}/actions/policies/{}", self.org, policy_id);
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// Handler for Actions Policies for a repository.
pub struct RepoActionsPoliciesHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> RepoActionsPoliciesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// List repository Actions policies.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#list-repository-actions-policies)
    pub async fn list(&self) -> Result<ActionsPoliciesResponse> {
        let route = format!("/repos/{}/{}/actions/policies", self.owner, self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Create a repository Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#create-a-repository-actions-policy)
    pub async fn create(&self, body: &impl Serialize) -> Result<ActionsPolicy> {
        let route = format!("/repos/{}/{}/actions/policies", self.owner, self.repo);
        self.crab.post(route, Some(body)).await
    }

    /// Get a repository Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#get-a-repository-actions-policy)
    pub async fn get(&self, policy_id: u64) -> Result<ActionsPolicy> {
        let route = format!(
            "/repos/{}/{}/actions/policies/{}",
            self.owner, self.repo, policy_id
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Update a repository Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#update-a-repository-actions-policy)
    pub async fn update(&self, policy_id: u64, body: &impl Serialize) -> Result<ActionsPolicy> {
        let route = format!(
            "/repos/{}/{}/actions/policies/{}",
            self.owner, self.repo, policy_id
        );
        self.crab.put(route, Some(body)).await
    }

    /// Delete a repository Actions policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/policies?apiVersion=2022-11-28#delete-a-repository-actions-policy)
    pub async fn delete(&self, policy_id: u64) -> Result<()> {
        let route = format!(
            "/repos/{}/{}/actions/policies/{}",
            self.owner, self.repo, policy_id
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
