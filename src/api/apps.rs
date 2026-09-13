use crate::{models::InstallationId, Octocrab};
use http::request::Builder;
use http::Method;

mod application;
mod installation_repositories;
mod installation_requests;
mod installations;
mod webhook;

pub use self::{
    application::ApplicationHandler,
    installation_repositories::ListInstallationRepositoriesBuilder,
    installation_requests::InstallationRequestsBuilder,
    installations::InstallationsRequestBuilder,
    webhook::{AppWebhookHandler, ListAppWebhookDeliveriesBuilder},
};

/// Type alias for [`AppsRequestHandler`].
pub type AppsHandler<'octo> = AppsRequestHandler<'octo>;

/// A client to [GitHub's apps API][apps-api].
///
/// Created with [`Octocrab::apps`].
///
/// [apps-api]: https://docs.github.com/en/rest/apps
pub struct AppsRequestHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> AppsRequestHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Gets the authenticated GitHub App.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#get-the-authenticated-app)
    pub async fn get(&self) -> crate::Result<crate::models::apps::App> {
        self.crab.get("/app", None::<&()>).await
    }

    /// Creates a GitHub App from a manifest code conversion.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-a-github-app-from-a-manifest)
    pub async fn create_from_manifest(
        &self,
        code: impl AsRef<str>,
    ) -> crate::Result<crate::models::apps::App> {
        let route = format!("/app-manifests/{}/conversions", code.as_ref());
        self.crab.post(route, None::<&()>).await
    }

    /// Get an installation for the authenticated app.
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::InstallationId;
    ///
    /// let installation = octocrab
    ///     .apps()
    ///     .installation(InstallationId(1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn installation(
        &self,
        installation_id: impl Into<InstallationId>,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!("/app/installations/{}", installation_id.into());
        self.crab.get(&route, None::<&()>).await
    }

    /// Creates a new `InstallationsBuilder` that can be configured to filter
    /// listing installations.
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let page = octocrab
    ///     .apps()
    ///     .installations()
    ///     // Optional Parameters
    ///     .since(chrono::Utc::now() - chrono::Duration::days(1))
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn installations(&self) -> installations::InstallationsRequestBuilder<'_, '_> {
        installations::InstallationsRequestBuilder::new(self)
    }

    /// Lists pending installation requests for the authenticated app.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#list-installation-requests-for-the-authenticated-app)
    pub fn installation_requests(&self) -> InstallationRequestsBuilder<'octo> {
        InstallationRequestsBuilder::new(self.crab)
    }

    /// Deletes an installation for the authenticated app.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#delete-an-installation-for-the-authenticated-app)
    pub async fn delete_installation(
        &self,
        installation_id: impl Into<InstallationId>,
    ) -> crate::Result<()> {
        let route = format!("/app/installations/{}", installation_id.into());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Creates an installation access token for an app.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-an-installation-access-token-for-an-app)
    pub async fn create_installation_access_token(
        &self,
        installation_id: impl Into<InstallationId>,
        body: &crate::models::apps::CreateInstallationAccessToken,
    ) -> crate::Result<crate::models::InstallationToken> {
        let route = format!(
            "/app/installations/{}/access_tokens",
            installation_id.into()
        );
        self.crab.post(route, Some(body)).await
    }

    /// Suspends an app installation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#suspend-an-app-installation)
    pub async fn suspend_installation(
        &self,
        installation_id: impl Into<InstallationId>,
    ) -> crate::Result<()> {
        let route = format!("/app/installations/{}/suspended", installation_id.into());
        let resp = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Unsuspends an app installation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#unsuspend-an-app-installation)
    pub async fn unsuspend_installation(
        &self,
        installation_id: impl Into<InstallationId>,
    ) -> crate::Result<()> {
        let route = format!("/app/installations/{}/suspended", installation_id.into());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Gets a user installation for the authenticated app.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#get-a-user-installation-for-the-authenticated-app)
    pub async fn get_user_installation(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!("/users/{}/installation", username.as_ref());
        self.crab.get(&route, None::<&()>).await
    }

    /// Lists repositories accessible to the app installation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#list-repositories-accessible-to-the-app-installation)
    pub fn installation_repositories(&self) -> ListInstallationRepositoriesBuilder<'octo> {
        ListInstallationRepositoriesBuilder::new(self.crab)
    }

    /// Revokes an installation access token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#revoke-an-installation-access-token)
    pub async fn revoke_installation_token(&self) -> crate::Result<()> {
        let resp = self
            .crab
            ._delete("/installation/token", None::<&()>)
            .await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Access webhook operations for the authenticated GitHub App.
    pub fn webhook(&self) -> AppWebhookHandler<'octo> {
        AppWebhookHandler::new(self.crab)
    }

    /// Gets the webhook configuration for the authenticated app.
    pub async fn webhook_config(&self) -> crate::Result<crate::models::hooks::Config> {
        self.webhook().get_config().await
    }

    /// Updates the webhook configuration for the authenticated app.
    pub async fn update_webhook_config(
        &self,
        config: &crate::models::apps::UpdateWebhookConfig,
    ) -> crate::Result<crate::models::hooks::Config> {
        self.webhook().update_config(config).await
    }

    /// Lists deliveries for the authenticated app's webhook.
    pub fn webhook_deliveries(&self) -> ListAppWebhookDeliveriesBuilder<'octo> {
        self.webhook().deliveries()
    }

    /// Gets a specific delivery for the authenticated app's webhook.
    pub async fn get_webhook_delivery(
        &self,
        delivery_id: impl Into<crate::models::HookDeliveryId>,
    ) -> crate::Result<crate::models::hooks::DeliveryDetail> {
        self.webhook().delivery(delivery_id).await
    }

    /// Redelivers a delivery for the authenticated app's webhook.
    pub async fn redeliver_webhook_delivery(
        &self,
        delivery_id: impl Into<crate::models::HookDeliveryId>,
    ) -> crate::Result<()> {
        self.webhook().redeliver(delivery_id).await
    }

    /// Access OAuth authorization operations for a specific application client ID.
    pub fn application(&self, client_id: impl Into<String>) -> ApplicationHandler<'octo> {
        ApplicationHandler::new(self.crab, client_id.into())
    }

    /// Access GitHub Marketplace operations.
    pub fn marketplace(&self) -> crate::api::marketplace::MarketplaceHandler<'octo> {
        crate::api::marketplace::MarketplaceHandler::new(self.crab)
    }

    pub(crate) async fn http_get<R, A, P>(
        &self,
        route: A,
        parameters: Option<&P>,
    ) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let request = Builder::new()
            .method(Method::GET)
            .uri(self.crab.parameterized_uri(route, parameters)?);
        let request = self.crab.build_request(request, None::<&()>)?;
        R::from_response(crate::map_github_error(self.crab.execute(request).await?).await?).await
    }

    /// Get a repository installation for the authenticated app.
    pub async fn get_repository_installation(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!(
            "/repos/{owner}/{repo}/installation",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );

        self.crab.get(&route, None::<&()>).await
    }

    /// Get an organization installation for the authenticated app.
    pub async fn get_org_installation(
        &self,
        owner: impl AsRef<str>,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!("/orgs/{owner}/installation", owner = owner.as_ref(),);

        self.crab.get(&route, None::<&()>).await
    }

    /// Get a GitHub App by its slug.
    pub async fn get_app(
        &self,
        app_slug: impl AsRef<str>,
    ) -> crate::Result<crate::models::apps::App> {
        let route = format!("/apps/{app_slug}", app_slug = app_slug.as_ref());

        self.crab.get(&route, None::<&()>).await
    }
}
