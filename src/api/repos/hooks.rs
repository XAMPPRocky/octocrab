use super::RepoHandler;
use crate::models::hooks::{Config, ContentType, Delivery, DeliveryDetail, Hook, UpdateHookConfig};
use crate::models::webhook_events::WebhookEventType;
use crate::models::{HookDeliveryId, HookId};
use crate::{Page, Result};

/// A client to GitHub's repository webhooks API.
///
/// Created with [`RepoHandler::hooks`].
pub struct RepoHooksHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoHooksHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists webhooks for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#list-repository-webhooks)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let hooks = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .list()
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListHooksBuilder<'octo, 'r> {
        ListHooksBuilder::new(self.handler)
    }

    /// Gets a webhook configured in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#get-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let hook = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .get(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, hook_id: impl Into<HookId>) -> Result<Hook> {
        let hook_id = hook_id.into();
        let route = format!("/{}/hooks/{hook_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a webhook for the specified repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#create-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::hooks::{Hook, Config as HookConfig, ContentType as HookContentType};
    ///
    /// let config = HookConfig {
    ///     url: "https://example.com/webhook".to_string(),
    ///     content_type: Some(HookContentType::Json),
    ///     insecure_ssl: None,
    ///     secret: None,
    /// };
    /// let hook = Hook {
    ///     name: "web".to_string(),
    ///     config,
    ///     ..Hook::default()
    /// };
    /// let hook = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .create(hook)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, hook: Hook) -> Result<Hook> {
        let route = format!("/{}/hooks", self.handler.repo);
        self.handler.crab.post(route, Some(&hook)).await
    }

    /// Creates an [`UpdateHookBuilder`] to update a webhook configured in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#update-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let hook = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .update(1)
    ///     .url("https://example.com/new-webhook")
    ///     .active(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, hook_id: impl Into<HookId>) -> UpdateHookBuilder<'octo, 'r> {
        UpdateHookBuilder::new(self.handler, hook_id.into())
    }

    /// Deletes a webhook from a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#delete-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .delete(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, hook_id: impl Into<HookId>) -> Result<()> {
        let hook_id = hook_id.into();
        let route = format!("/{}/hooks/{hook_id}", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Returns the webhook configuration for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#get-a-webhook-configuration-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let config = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .get_config(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_config(&self, hook_id: impl Into<HookId>) -> Result<Config> {
        let hook_id = hook_id.into();
        let route = format!("/{}/hooks/{hook_id}/config", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates an [`UpdateHookConfigBuilder`] to update a webhook configuration for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#update-a-webhook-configuration-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::hooks::ContentType;
    ///
    /// let config = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .update_config(1)
    ///     .url("https://example.com/updated-webhook")
    ///     .content_type(ContentType::Json)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_config(&self, hook_id: impl Into<HookId>) -> UpdateHookConfigBuilder<'octo, 'r> {
        UpdateHookConfigBuilder::new(self.handler, hook_id.into())
    }

    /// Triggers a ping event to be sent to the webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#ping-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .ping(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ping(&self, hook_id: impl Into<HookId>) -> Result<()> {
        let hook_id = hook_id.into();
        let route = format!("/{}/hooks/{hook_id}/pings", self.handler.repo);
        let response = self.handler.crab._post(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Triggers the push webhook test with the latest push to the current repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#test-the-push-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .test(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn test(&self, hook_id: impl Into<HookId>) -> Result<()> {
        let hook_id = hook_id.into();
        let route = format!("/{}/hooks/{hook_id}/tests", self.handler.repo);
        let response = self.handler.crab._post(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Creates a [`RepoHookDeliveriesHandler`] for managing deliveries of the specified webhook.
    pub fn deliveries(&self, hook_id: impl Into<HookId>) -> RepoHookDeliveriesHandler<'octo, 'r> {
        RepoHookDeliveriesHandler::new(self.handler, hook_id.into())
    }

    /// Gets a delivery for a webhook configured in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#get-a-delivery-for-a-repository-webhook)
    pub async fn get_delivery(
        &self,
        hook_id: impl Into<HookId>,
        delivery_id: impl Into<HookDeliveryId>,
    ) -> Result<DeliveryDetail> {
        self.deliveries(hook_id).get(delivery_id).await
    }

    /// Redelivers a webhook delivery for a webhook configured in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#redeliver-a-delivery-for-a-repository-webhook)
    pub async fn redeliver(
        &self,
        hook_id: impl Into<HookId>,
        delivery_id: impl Into<HookDeliveryId>,
    ) -> Result<()> {
        self.deliveries(hook_id).redeliver(delivery_id).await
    }
}

/// A client for managing deliveries of a specific repository webhook.
pub struct RepoHookDeliveriesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    hook_id: HookId,
}

impl<'octo, 'r> RepoHookDeliveriesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, hook_id: HookId) -> Self {
        Self { handler, hook_id }
    }

    /// Lists deliveries for this webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#list-deliveries-for-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let deliveries = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .deliveries(1)
    ///     .list()
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListRepoHookDeliveriesBuilder<'octo, 'r> {
        ListRepoHookDeliveriesBuilder::new(self.handler, self.hook_id)
    }

    /// Gets a delivery for this webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#get-a-delivery-for-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let delivery = octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .deliveries(1)
    ///     .get(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, delivery_id: impl Into<HookDeliveryId>) -> Result<DeliveryDetail> {
        let delivery_id = delivery_id.into();
        let route = format!(
            "/{}/hooks/{}/deliveries/{delivery_id}",
            self.handler.repo, self.hook_id
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Redelivers a delivery for this webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#redeliver-a-delivery-for-a-repository-webhook)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.repos("owner", "repo")
    ///     .hooks()
    ///     .deliveries(1)
    ///     .redeliver(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn redeliver(&self, delivery_id: impl Into<HookDeliveryId>) -> Result<()> {
        let delivery_id = delivery_id.into();
        let route = format!(
            "/{}/hooks/{}/deliveries/{delivery_id}/attempts",
            self.handler.repo, self.hook_id
        );
        let response = self.handler.crab._post(route, None::<&()>).await?;
        if response.status() != http::StatusCode::ACCEPTED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for listing repository webhooks.
#[derive(serde::Serialize)]
pub struct ListHooksBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListHooksBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<Hook>> {
        let route = format!("/{}/hooks", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for listing deliveries of a repository webhook.
#[derive(serde::Serialize)]
pub struct ListRepoHookDeliveriesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    hook_id: HookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}

impl<'octo, 'r> ListRepoHookDeliveriesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, hook_id: HookId) -> Self {
        Self {
            handler,
            hook_id,
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
    pub async fn send(self) -> Result<Page<Delivery>> {
        let route = format!("/{}/hooks/{}/deliveries", self.handler.repo, self.hook_id);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating a repository webhook.
#[derive(serde::Serialize)]
pub struct UpdateHookBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    hook_id: HookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<UpdateHookConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_events: Option<Vec<WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_events: Option<Vec<WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
}

impl<'octo, 'r> UpdateHookBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, hook_id: HookId) -> Self {
        Self {
            handler,
            hook_id,
            config: None,
            events: None,
            add_events: None,
            remove_events: None,
            active: None,
        }
    }

    /// Sets the entire configuration object of the webhook.
    pub fn config(mut self, config: UpdateHookConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Sets the URL to which the payloads will be delivered.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.url = Some(url.into());
        self.config = Some(config);
        self
    }

    /// Sets the media type used to serialize the payloads.
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.content_type = Some(content_type);
        self.config = Some(config);
        self
    }

    /// Sets the secret key used for HMAC signature generation.
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.secret = Some(secret.into());
        self.config = Some(config);
        self
    }

    /// Determines whether the SSL certificate of the host will be verified.
    pub fn insecure_ssl(mut self, insecure_ssl: impl Into<String>) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.insecure_ssl = Some(insecure_ssl.into());
        self.config = Some(config);
        self
    }

    /// Determines what events the hook is triggered for. This replaces the entire array of events.
    pub fn events(mut self, events: impl Into<Vec<WebhookEventType>>) -> Self {
        self.events = Some(events.into());
        self
    }

    /// Determines a list of events to be added to the list of events that the Hook triggers for.
    pub fn add_events(mut self, events: impl Into<Vec<WebhookEventType>>) -> Self {
        self.add_events = Some(events.into());
        self
    }

    /// Determines a list of events to be removed from the list of events that the Hook triggers for.
    pub fn remove_events(mut self, events: impl Into<Vec<WebhookEventType>>) -> Self {
        self.remove_events = Some(events.into());
        self
    }

    /// Determines if notifications are sent when the webhook is triggered.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Hook> {
        let route = format!("/{}/hooks/{}", self.handler.repo, self.hook_id);
        self.handler.crab.patch(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating a webhook configuration.
#[derive(serde::Serialize)]
pub struct UpdateHookConfigBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    hook_id: HookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_ssl: Option<String>,
}

impl<'octo, 'r> UpdateHookConfigBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, hook_id: HookId) -> Self {
        Self {
            handler,
            hook_id,
            url: None,
            content_type: None,
            secret: None,
            insecure_ssl: None,
        }
    }

    /// The URL to which the payloads will be delivered.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// The media type used to serialize the payloads.
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// If provided, the secret will be used as the key to generate HMAC hex digest values.
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    /// Determines whether the SSL certificate of the host will be verified.
    pub fn insecure_ssl(mut self, insecure_ssl: impl Into<String>) -> Self {
        self.insecure_ssl = Some(insecure_ssl.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Config> {
        let route = format!("/{}/hooks/{}/config", self.handler.repo, self.hook_id);
        self.handler.crab.patch(route, Some(&self)).await
    }
}
