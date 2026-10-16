use serde::Serialize;

use crate::api::orgs::OrgHandler;
use crate::Result;

/// Handler for organization external custom properties API.
pub struct OrgExternalPropertiesHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgExternalPropertiesHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Get registered app installations for external custom properties.
    pub async fn list_installations(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/properties/installations", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Register an app installation for external custom properties.
    pub async fn register_installation(&self, body: &impl Serialize) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/properties/installations", self.handler.owner);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get all external custom properties for a GitHub App installation in an organization.
    pub async fn get_schema(&self) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/properties/installations/schema",
            self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create or update external custom property values for organization repositories.
    pub async fn create_or_update_values(&self, body: &impl Serialize) -> Result<()> {
        let route = format!(
            "/orgs/{}/properties/installations/values",
            self.handler.owner
        );
        let res = self.handler.crab._patch(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Create or update external custom property values for a property across organization repositories.
    pub async fn create_or_update_property_values(
        &self,
        property_name: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/properties/installations/values/{}",
            self.handler.owner,
            property_name.as_ref()
        );
        let res = self.handler.crab._patch(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Remove all external custom property values for a property across all organization repositories.
    pub async fn delete_property_values(&self, property_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/properties/installations/values/{}",
            self.handler.owner,
            property_name.as_ref()
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}
