use crate::models::apps::UpdateWebhookConfig;
use crate::models::hooks::{Config, Delivery, DeliveryDetail};
use crate::models::HookDeliveryId;
use crate::{Octocrab, Result};

/// A client for managing webhook configuration and deliveries for the authenticated GitHub App.
///
/// Created with [`AppsRequestHandler::webhook`][super::AppsRequestHandler::webhook].
pub struct AppWebhookHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> AppWebhookHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Gets the webhook configuration for the authenticated app.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/webhooks?apiVersion=2022-11-28#get-a-webhook-configuration-for-an-app)
    pub async fn get_config(&self) -> Result<Config> {
        self.crab.get("/app/hook/config", None::<&()>).await
    }

    /// Updates the webhook configuration for the authenticated app.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/webhooks?apiVersion=2022-11-28#update-a-webhook-configuration-for-an-app)
    pub async fn update_config(&self, config: &UpdateWebhookConfig) -> Result<Config> {
        self.crab.patch("/app/hook/config", Some(config)).await
    }

    /// Lists deliveries for the authenticated app's webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/webhooks?apiVersion=2022-11-28#list-deliveries-for-an-app-webhook)
    pub fn deliveries(&self) -> ListAppWebhookDeliveriesBuilder<'octo> {
        ListAppWebhookDeliveriesBuilder::new(self.crab)
    }

    /// Gets a specific delivery for the authenticated app's webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/webhooks?apiVersion=2022-11-28#get-a-delivery-for-an-app-webhook)
    pub async fn delivery(&self, delivery_id: impl Into<HookDeliveryId>) -> Result<DeliveryDetail> {
        let route = format!("/app/hook/deliveries/{}", delivery_id.into());
        self.crab.get(route, None::<&()>).await
    }

    /// Redelivers a delivery for the authenticated app's webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/webhooks?apiVersion=2022-11-28#redeliver-a-delivery-for-an-app-webhook)
    pub async fn redeliver(&self, delivery_id: impl Into<HookDeliveryId>) -> Result<()> {
        let route = format!("/app/hook/deliveries/{}/attempts", delivery_id.into());
        let resp = self.crab._post(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}

/// A builder pattern struct for listing app webhook deliveries.
///
/// Created by [`AppWebhookHandler::deliveries`].
#[derive(serde::Serialize)]
pub struct ListAppWebhookDeliveriesBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}

impl<'octo> ListAppWebhookDeliveriesBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            per_page: None,
            cursor: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Used for pagination: the starting delivery from which the page of deliveries is fetched.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Vec<Delivery>> {
        self.crab.get("/app/hook/deliveries", Some(&self)).await
    }
}
