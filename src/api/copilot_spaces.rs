use serde::Serialize;

use crate::models::copilot_spaces::{
    CopilotSpace, CopilotSpaceCollaboratorsResponse, CopilotSpaceResource,
    CopilotSpaceResourcesResponse, CopilotSpacesResponse,
};
use crate::{Octocrab, Result};

/// A client to GitHub's Copilot Spaces API.
///
/// Created with [`Octocrab::copilot_spaces`].
pub struct CopilotSpacesHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CopilotSpacesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Access Copilot Spaces for an organization.
    pub fn org<O: Into<String>>(&self, org: O) -> OrgCopilotSpacesHandler<'octo> {
        OrgCopilotSpacesHandler {
            crab: self.crab,
            org: org.into(),
        }
    }

    /// Access Copilot Spaces for a user.
    pub fn user<U: Into<String>>(&self, user: U) -> UserCopilotSpacesHandler<'octo> {
        UserCopilotSpacesHandler {
            crab: self.crab,
            user: user.into(),
        }
    }
}

/// Handler for organization-scoped Copilot Spaces.
pub struct OrgCopilotSpacesHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> OrgCopilotSpacesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: impl Into<String>) -> Self {
        Self {
            crab,
            org: org.into(),
        }
    }

    /// List organization Copilot Spaces.
    pub async fn list(&self) -> Result<CopilotSpacesResponse> {
        let route = format!("/orgs/{}/copilot-spaces", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Create an organization Copilot Space.
    pub async fn create(&self, body: &impl Serialize) -> Result<CopilotSpace> {
        let route = format!("/orgs/{}/copilot-spaces", self.org);
        self.crab.post(route, Some(body)).await
    }

    /// Get an organization Copilot Space.
    pub async fn get(&self, space_number: u64) -> Result<CopilotSpace> {
        let route = format!("/orgs/{}/copilot-spaces/{}", self.org, space_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Set an organization Copilot Space.
    pub async fn set(&self, space_number: u64, body: &impl Serialize) -> Result<CopilotSpace> {
        let route = format!("/orgs/{}/copilot-spaces/{}", self.org, space_number);
        self.crab.put(route, Some(body)).await
    }

    /// Delete an organization Copilot Space.
    pub async fn delete(&self, space_number: u64) -> Result<()> {
        let route = format!("/orgs/{}/copilot-spaces/{}", self.org, space_number);
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List collaborators for an organization Copilot Space.
    pub async fn list_collaborators(
        &self,
        space_number: u64,
    ) -> Result<CopilotSpaceCollaboratorsResponse> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/collaborators",
            self.org, space_number
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Add a collaborator to an organization Copilot Space.
    pub async fn add_collaborator(&self, space_number: u64, body: &impl Serialize) -> Result<()> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/collaborators",
            self.org, space_number
        );
        let res = self.crab._post(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Set a collaborator role for an organization Copilot Space.
    pub async fn set_collaborator_role(
        &self,
        space_number: u64,
        actor_type: impl AsRef<str>,
        actor_identifier: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/collaborators/{}/{}",
            self.org,
            space_number,
            actor_type.as_ref(),
            actor_identifier.as_ref()
        );
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Remove a collaborator from an organization Copilot Space.
    pub async fn remove_collaborator(
        &self,
        space_number: u64,
        actor_type: impl AsRef<str>,
        actor_identifier: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/collaborators/{}/{}",
            self.org,
            space_number,
            actor_type.as_ref(),
            actor_identifier.as_ref()
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List resources for an organization Copilot Space.
    pub async fn list_resources(&self, space_number: u64) -> Result<CopilotSpaceResourcesResponse> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/resources",
            self.org, space_number
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Create a resource for an organization Copilot Space.
    pub async fn create_resource(
        &self,
        space_number: u64,
        body: &impl Serialize,
    ) -> Result<CopilotSpaceResource> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/resources",
            self.org, space_number
        );
        self.crab.post(route, Some(body)).await
    }

    /// Get a resource for an organization Copilot Space.
    pub async fn get_resource(
        &self,
        space_number: u64,
        space_resource_id: u64,
    ) -> Result<CopilotSpaceResource> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/resources/{}",
            self.org, space_number, space_resource_id
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set a resource for an organization Copilot Space.
    pub async fn set_resource(
        &self,
        space_number: u64,
        space_resource_id: u64,
        body: &impl Serialize,
    ) -> Result<CopilotSpaceResource> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/resources/{}",
            self.org, space_number, space_resource_id
        );
        self.crab.put(route, Some(body)).await
    }

    /// Delete a resource from an organization Copilot Space.
    pub async fn delete_resource(&self, space_number: u64, space_resource_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/copilot-spaces/{}/resources/{}",
            self.org, space_number, space_resource_id
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

/// Handler for user-scoped Copilot Spaces.
pub struct UserCopilotSpacesHandler<'octo> {
    crab: &'octo Octocrab,
    user: String,
}

impl<'octo> UserCopilotSpacesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, user: impl Into<String>) -> Self {
        Self {
            crab,
            user: user.into(),
        }
    }

    /// List Copilot Spaces for a user.
    pub async fn list(&self) -> Result<CopilotSpacesResponse> {
        let route = format!("/users/{}/copilot-spaces", self.user);
        self.crab.get(route, None::<&()>).await
    }

    /// Create a Copilot Space for a user.
    pub async fn create(&self, body: &impl Serialize) -> Result<CopilotSpace> {
        let route = format!("/users/{}/copilot-spaces", self.user);
        self.crab.post(route, Some(body)).await
    }

    /// Get a Copilot Space for a user.
    pub async fn get(&self, space_number: u64) -> Result<CopilotSpace> {
        let route = format!("/users/{}/copilot-spaces/{}", self.user, space_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Set a Copilot Space for a user.
    pub async fn set(&self, space_number: u64, body: &impl Serialize) -> Result<CopilotSpace> {
        let route = format!("/users/{}/copilot-spaces/{}", self.user, space_number);
        self.crab.put(route, Some(body)).await
    }

    /// Delete a Copilot Space for a user.
    pub async fn delete(&self, space_number: u64) -> Result<()> {
        let route = format!("/users/{}/copilot-spaces/{}", self.user, space_number);
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List collaborators for a user Copilot Space.
    pub async fn list_collaborators(
        &self,
        space_number: u64,
    ) -> Result<CopilotSpaceCollaboratorsResponse> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/collaborators",
            self.user, space_number
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Add a collaborator to a user Copilot Space.
    pub async fn add_collaborator(&self, space_number: u64, body: &impl Serialize) -> Result<()> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/collaborators",
            self.user, space_number
        );
        let res = self.crab._post(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Set a collaborator role for a user Copilot Space.
    pub async fn set_collaborator_role(
        &self,
        space_number: u64,
        actor_type: impl AsRef<str>,
        actor_identifier: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/collaborators/{}/{}",
            self.user,
            space_number,
            actor_type.as_ref(),
            actor_identifier.as_ref()
        );
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Remove a collaborator from a user Copilot Space.
    pub async fn remove_collaborator(
        &self,
        space_number: u64,
        actor_type: impl AsRef<str>,
        actor_identifier: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/collaborators/{}/{}",
            self.user,
            space_number,
            actor_type.as_ref(),
            actor_identifier.as_ref()
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List resources for a user Copilot Space.
    pub async fn list_resources(&self, space_number: u64) -> Result<CopilotSpaceResourcesResponse> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/resources",
            self.user, space_number
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Create a resource for a user Copilot Space.
    pub async fn create_resource(
        &self,
        space_number: u64,
        body: &impl Serialize,
    ) -> Result<CopilotSpaceResource> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/resources",
            self.user, space_number
        );
        self.crab.post(route, Some(body)).await
    }

    /// Get a resource for a user Copilot Space.
    pub async fn get_resource(
        &self,
        space_number: u64,
        space_resource_id: u64,
    ) -> Result<CopilotSpaceResource> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/resources/{}",
            self.user, space_number, space_resource_id
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set a resource for a user Copilot Space.
    pub async fn set_resource(
        &self,
        space_number: u64,
        space_resource_id: u64,
        body: &impl Serialize,
    ) -> Result<CopilotSpaceResource> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/resources/{}",
            self.user, space_number, space_resource_id
        );
        self.crab.put(route, Some(body)).await
    }

    /// Delete a resource from a user Copilot Space.
    pub async fn delete_resource(&self, space_number: u64, space_resource_id: u64) -> Result<()> {
        let route = format!(
            "/users/{}/copilot-spaces/{}/resources/{}",
            self.user, space_number, space_resource_id
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}
