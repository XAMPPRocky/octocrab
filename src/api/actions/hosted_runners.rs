use crate::models::actions::{HostedRunner, HostedRunnersResponse};
use crate::{Octocrab, Result};
use serde::Serialize;

/// Handler for GitHub Actions Hosted Runners for an organization.
pub struct OrgHostedRunnersHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> OrgHostedRunnersHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String) -> Self {
        Self { crab, org }
    }

    /// List GitHub-hosted runners for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#list-github-hosted-runners-for-an-organization)
    pub async fn list(&self) -> Result<HostedRunnersResponse> {
        let route = format!("/orgs/{}/actions/hosted-runners", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Create a GitHub-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#create-a-github-hosted-runner-for-an-organization)
    pub async fn create(&self, body: &impl Serialize) -> Result<HostedRunner> {
        let route = format!("/orgs/{}/actions/hosted-runners", self.org);
        self.crab.post(route, Some(body)).await
    }

    /// Get a GitHub-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-a-github-hosted-runner-for-an-organization)
    pub async fn get(&self, hosted_runner_id: u64) -> Result<HostedRunner> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/{}",
            self.org, hosted_runner_id
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Update a GitHub-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#update-a-github-hosted-runner-for-an-organization)
    pub async fn update(
        &self,
        hosted_runner_id: u64,
        body: &impl Serialize,
    ) -> Result<HostedRunner> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/{}",
            self.org, hosted_runner_id
        );
        self.crab.patch(route, Some(body)).await
    }

    /// Delete a GitHub-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#delete-a-github-hosted-runner-for-an-organization)
    pub async fn delete(&self, hosted_runner_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/{}",
            self.org, hosted_runner_id
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// List custom images for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#list-custom-images-for-an-organization)
    pub async fn list_custom_images(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/actions/hosted-runners/images/custom", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Get a custom image definition for GitHub Actions Hosted Runners.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-a-custom-image-definition-for-github-actions-hosted-runners)
    pub async fn get_custom_image(
        &self,
        image_definition_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/images/custom/{}",
            self.org,
            image_definition_id.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Delete a custom image from the organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#delete-a-custom-image-from-the-organization)
    pub async fn delete_custom_image(&self, image_definition_id: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/images/custom/{}",
            self.org,
            image_definition_id.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// List image versions of a custom image for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#list-image-versions-of-a-custom-image-for-an-organization)
    pub async fn list_custom_image_versions(
        &self,
        image_definition_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/images/custom/{}/versions",
            self.org,
            image_definition_id.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get an image version of a custom image for GitHub Actions Hosted Runners.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-an-image-version-of-a-custom-image-for-github-actions-hosted-runners)
    pub async fn get_custom_image_version(
        &self,
        image_definition_id: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/images/custom/{}/versions/{}",
            self.org,
            image_definition_id.as_ref(),
            version.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Delete an image version of custom image from the organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#delete-an-image-version-of-custom-image-from-the-organization)
    pub async fn delete_custom_image_version(
        &self,
        image_definition_id: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/images/custom/{}/versions/{}",
            self.org,
            image_definition_id.as_ref(),
            version.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Get GitHub-owned images for GitHub-hosted runners in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-github-owned-images-for-github-hosted-runners-in-an-organization)
    pub async fn list_github_owned_images(&self) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/hosted-runners/images/github-owned",
            self.org
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get partner images for GitHub-hosted runners in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-partner-images-for-github-hosted-runners-in-an-organization)
    pub async fn list_partner_images(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/actions/hosted-runners/images/partner", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Get limits on GitHub-hosted runners for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-limits-on-github-hosted-runners-for-an-organization)
    pub async fn get_limits(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/actions/hosted-runners/limits", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Get GitHub-hosted runners machine specs for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-github-hosted-runners-machine-specs-for-an-organization)
    pub async fn get_machine_sizes(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/actions/hosted-runners/machine-sizes", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Get platforms for GitHub-hosted runners in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/hosted-runners?apiVersion=2022-11-28#get-platforms-for-github-hosted-runners-in-an-organization)
    pub async fn get_platforms(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/actions/hosted-runners/platforms", self.org);
        self.crab.get(route, None::<&()>).await
    }
}
