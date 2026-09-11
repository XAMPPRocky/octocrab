use super::OrgHandler;
use crate::models::hooks::{Config, Hook, UpdateHookConfig};
use crate::models::webhook_events::WebhookEventType;
use crate::models::HookId;
use crate::{Page, Result};

/// A client to GitHub's organization webhooks API.
///
/// Created with [`OrgHandler::hooks`].
pub struct OrgHooksHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgHooksHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists webhooks for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#list-organization-webhooks)
    pub fn list(&self) -> ListOrgHooksBuilder<'octo, 'r> {
        ListOrgHooksBuilder::new(self.handler)
    }

    /// Gets a webhook configured in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#get-an-organization-webhook)
    pub async fn get(&self, hook_id: impl Into<HookId>) -> Result<Hook> {
        let hook_id = hook_id.into();
        let route = format!("/orgs/{org}/hooks/{hook_id}", org = self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a webhook for the specified organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#create-an-organization-webhook)
    pub async fn create(&self, hook: Hook) -> Result<Hook> {
        let route = format!("/orgs/{org}/hooks", org = self.handler.owner);
        self.handler.crab.post(route, Some(&hook)).await
    }

    /// Updates a webhook configured in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#update-an-organization-webhook)
    pub fn update(&self, hook_id: impl Into<HookId>) -> UpdateOrgHookBuilder<'octo, 'r> {
        UpdateOrgHookBuilder::new(self.handler, hook_id.into())
    }

    /// Deletes a webhook configured in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#delete-an-organization-webhook)
    pub async fn delete(&self, hook_id: impl Into<HookId>) -> Result<()> {
        let hook_id = hook_id.into();
        let route = format!("/orgs/{org}/hooks/{hook_id}", org = self.handler.owner);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Pings an organization webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#ping-an-organization-webhook)
    pub async fn ping(&self, hook_id: impl Into<HookId>) -> Result<()> {
        let hook_id = hook_id.into();
        let route = format!(
            "/orgs/{org}/hooks/{hook_id}/pings",
            org = self.handler.owner
        );
        let response = self.handler.crab._post(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Gets the configuration for an organization webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#get-a-webhook-configuration-for-an-organization)
    pub async fn get_config(&self, hook_id: impl Into<HookId>) -> Result<Config> {
        let hook_id = hook_id.into();
        let route = format!(
            "/orgs/{org}/hooks/{hook_id}/config",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Updates the configuration for an organization webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28#update-a-webhook-configuration-for-an-organization)
    pub async fn update_config(
        &self,
        hook_id: impl Into<HookId>,
        config: UpdateHookConfig,
    ) -> Result<Config> {
        let hook_id = hook_id.into();
        let route = format!(
            "/orgs/{org}/hooks/{hook_id}/config",
            org = self.handler.owner
        );
        self.handler.crab.patch(route, Some(&config)).await
    }
}

/// Builder for listing organization webhooks.
#[derive(serde::Serialize)]
pub struct ListOrgHooksBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgHooksBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> Result<Page<Hook>> {
        let route = format!("/orgs/{org}/hooks", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for updating an organization webhook.
#[derive(serde::Serialize)]
pub struct UpdateOrgHookBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip)]
    hook_id: HookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<UpdateHookConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl<'octo, 'r> UpdateOrgHookBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>, hook_id: HookId) -> Self {
        Self {
            handler,
            hook_id,
            config: None,
            events: None,
            active: None,
            name: None,
        }
    }

    pub fn config(mut self, config: UpdateHookConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn events(mut self, events: impl Into<Vec<WebhookEventType>>) -> Self {
        self.events = Some(events.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub async fn send(self) -> Result<Hook> {
        let route = format!(
            "/orgs/{org}/hooks/{hook_id}",
            org = self.handler.owner,
            hook_id = self.hook_id
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}
