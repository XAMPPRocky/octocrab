use super::OrgHandler;
use crate::models::orgs::orgs_extensions::{
    OrgImmutableReleasesRepositoriesResponse, OrgImmutableReleasesSettings,
};
use crate::Result;

/// Handler for GitHub's Organization Immutable Releases API.
///
/// Created with [`OrgHandler::immutable_releases`].
pub struct OrgImmutableReleasesHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgImmutableReleasesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Get immutable releases settings for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-settings?apiVersion=2022-11-28#get-immutable-releases-settings-for-an-organization)
    pub async fn get_settings(&self) -> Result<OrgImmutableReleasesSettings> {
        let route = format!(
            "/orgs/{org}/settings/immutable-releases",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Set immutable releases settings for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-settings?apiVersion=2022-11-28#set-immutable-releases-settings-for-an-organization)
    pub async fn set_settings(&self, settings: &impl serde::Serialize) -> Result<()> {
        let route = format!(
            "/orgs/{org}/settings/immutable-releases",
            org = self.handler.owner
        );
        let response = self.handler.crab._put(route, Some(settings)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// List selected repositories for immutable releases enforcement.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-settings?apiVersion=2022-11-28#list-selected-repositories-for-immutable-releases-enforcement)
    pub async fn list_repositories(&self) -> Result<OrgImmutableReleasesRepositoriesResponse> {
        let route = format!(
            "/orgs/{org}/settings/immutable-releases/repositories",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Set selected repositories for immutable releases enforcement.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-settings?apiVersion=2022-11-28#set-selected-repositories-for-immutable-releases-enforcement)
    pub async fn set_repositories(&self, repositories: &impl serde::Serialize) -> Result<()> {
        let route = format!(
            "/orgs/{org}/settings/immutable-releases/repositories",
            org = self.handler.owner
        );
        let response = self.handler.crab._put(route, Some(repositories)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Enable a selected repository for immutable releases in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-settings?apiVersion=2022-11-28#enable-a-selected-repository-for-immutable-releases-in-an-organization)
    pub async fn enable_repository(&self, repository_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{org}/settings/immutable-releases/repositories/{repository_id}",
            org = self.handler.owner
        );
        let response = self.handler.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Disable a selected repository for immutable releases in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-settings?apiVersion=2022-11-28#disable-a-selected-repository-for-immutable-releases-in-an-organization)
    pub async fn disable_repository(&self, repository_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{org}/settings/immutable-releases/repositories/{repository_id}",
            org = self.handler.owner
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
