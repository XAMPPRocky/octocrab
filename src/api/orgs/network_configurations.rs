use serde::Serialize;

use crate::api::orgs::OrgHandler;
use crate::models::orgs::{
    OrgNetworkConfiguration, OrgNetworkConfigurationsResponse, OrgNetworkSettings,
};
use crate::Result;

/// Handler for organization network configurations API (hosted compute).
pub struct OrgNetworkConfigurationsHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgNetworkConfigurationsHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List hosted compute network configurations for an organization.
    pub async fn list(&self) -> Result<OrgNetworkConfigurationsResponse> {
        let route = format!(
            "/orgs/{}/settings/network-configurations",
            self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create a hosted compute network configuration for an organization.
    pub async fn create(&self, body: &impl Serialize) -> Result<OrgNetworkConfiguration> {
        let route = format!(
            "/orgs/{}/settings/network-configurations",
            self.handler.owner
        );
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get a hosted compute network configuration for an organization.
    pub async fn get(&self, configuration_id: impl AsRef<str>) -> Result<OrgNetworkConfiguration> {
        let route = format!(
            "/orgs/{}/settings/network-configurations/{}",
            self.handler.owner,
            configuration_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a hosted compute network configuration for an organization.
    pub async fn update(
        &self,
        configuration_id: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<OrgNetworkConfiguration> {
        let route = format!(
            "/orgs/{}/settings/network-configurations/{}",
            self.handler.owner,
            configuration_id.as_ref()
        );
        self.handler.crab.patch(route, Some(body)).await
    }

    /// Delete a hosted compute network configuration from an organization.
    pub async fn delete(&self, configuration_id: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/settings/network-configurations/{}",
            self.handler.owner,
            configuration_id.as_ref()
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Get a hosted compute network settings resource for an organization.
    pub async fn get_network_settings(
        &self,
        settings_id: impl AsRef<str>,
    ) -> Result<OrgNetworkSettings> {
        let route = format!(
            "/orgs/{}/settings/network-settings/{}",
            self.handler.owner,
            settings_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}
