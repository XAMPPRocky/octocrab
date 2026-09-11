use super::OrgHandler;
use crate::models::orgs::roles::{OrgFineGrainedPermission, OrgRole, OrgRolesResponse};
use crate::models::teams::Team;
use crate::models::{Author, OrgRoleId};
use crate::Result;

/// A client to GitHub's organization roles API.
///
/// Created with [`OrgHandler::roles`].
pub struct OrgRolesHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgRolesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Gets all organization roles for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#get-all-organization-roles-for-an-organization)
    pub async fn list(&self) -> Result<OrgRolesResponse> {
        let route = format!("/orgs/{org}/organization-roles", org = self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a custom organization role.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#create-a-custom-organization-role)
    pub async fn create(
        &self,
        name: impl Into<String>,
        description: Option<String>,
        permissions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<OrgRole> {
        let route = format!("/orgs/{org}/organization-roles", org = self.handler.owner);
        let body = serde_json::json!({
            "name": name.into(),
            "description": description,
            "permissions": permissions.into_iter().map(Into::into).collect::<Vec<_>>()
        });
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Gets an organization role.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#get-an-organization-role)
    pub async fn get(&self, role_id: impl Into<OrgRoleId>) -> Result<OrgRole> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/{role_id}",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Updates an organization role.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#update-an-organization-role)
    pub async fn update(
        &self,
        role_id: impl Into<OrgRoleId>,
        name: Option<String>,
        description: Option<String>,
        permissions: Option<Vec<String>>,
    ) -> Result<OrgRole> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/{role_id}",
            org = self.handler.owner
        );
        let mut body = serde_json::Map::new();
        if let Some(name) = name {
            body.insert("name".to_string(), serde_json::Value::String(name));
        }
        if let Some(description) = description {
            body.insert(
                "description".to_string(),
                serde_json::Value::String(description),
            );
        }
        if let Some(permissions) = permissions {
            body.insert(
                "permissions".to_string(),
                serde_json::to_value(permissions).unwrap(),
            );
        }
        self.handler.crab.patch(route, Some(&body)).await
    }

    /// Deletes an organization role.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#delete-an-organization-role)
    pub async fn delete(&self, role_id: impl Into<OrgRoleId>) -> Result<()> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/{role_id}",
            org = self.handler.owner
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists teams assigned to an organization role.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#list-teams-that-are-assigned-to-an-organization-role)
    pub async fn list_teams_for_role(&self, role_id: impl Into<OrgRoleId>) -> Result<Vec<Team>> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/{role_id}/teams",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Lists users assigned to an organization role.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#list-users-that-are-assigned-to-an-organization-role)
    pub async fn list_users_for_role(&self, role_id: impl Into<OrgRoleId>) -> Result<Vec<Author>> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/{role_id}/users",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Lists organization roles assigned to a team.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#list-organization-roles-assigned-to-a-team)
    pub async fn list_roles_for_team(
        &self,
        team_slug: impl AsRef<str>,
    ) -> Result<OrgRolesResponse> {
        let route = format!(
            "/orgs/{org}/organization-roles/teams/{team_slug}",
            org = self.handler.owner,
            team_slug = team_slug.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Assigns an organization role to a team.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#assign-an-organization-role-to-a-team)
    pub async fn assign_to_team(
        &self,
        team_slug: impl AsRef<str>,
        role_id: impl Into<OrgRoleId>,
    ) -> Result<()> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}",
            org = self.handler.owner,
            team_slug = team_slug.as_ref()
        );
        let response = self.handler.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Removes an organization role from a team.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#remove-an-organization-role-from-a-team)
    pub async fn remove_from_team(
        &self,
        team_slug: impl AsRef<str>,
        role_id: impl Into<OrgRoleId>,
    ) -> Result<()> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}",
            org = self.handler.owner,
            team_slug = team_slug.as_ref()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists organization roles assigned to a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#list-organization-roles-assigned-to-a-user)
    pub async fn list_roles_for_user(&self, username: impl AsRef<str>) -> Result<OrgRolesResponse> {
        let route = format!(
            "/orgs/{org}/organization-roles/users/{username}",
            org = self.handler.owner,
            username = username.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Assigns an organization role to a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#assign-an-organization-role-to-a-user)
    pub async fn assign_to_user(
        &self,
        username: impl AsRef<str>,
        role_id: impl Into<OrgRoleId>,
    ) -> Result<()> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/users/{username}/{role_id}",
            org = self.handler.owner,
            username = username.as_ref()
        );
        let response = self.handler.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Removes an organization role from a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#remove-an-organization-role-from-a-user)
    pub async fn remove_from_user(
        &self,
        username: impl AsRef<str>,
        role_id: impl Into<OrgRoleId>,
    ) -> Result<()> {
        let role_id = role_id.into();
        let route = format!(
            "/orgs/{org}/organization-roles/users/{username}/{role_id}",
            org = self.handler.owner,
            username = username.as_ref()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Gets all organization fine-grained permissions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28#get-all-organization-fine-grained-permissions)
    pub async fn list_fine_grained_permissions(&self) -> Result<Vec<OrgFineGrainedPermission>> {
        let route = format!(
            "/orgs/{org}/organization-fine-grained-permissions",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}
