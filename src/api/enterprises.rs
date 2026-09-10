//! The Enterprise API.

use crate::api::repos::secret_scanning_alerts::Params;
use crate::Octocrab;

/// A client to GitHub's enterprise API.
///
/// Created with [`Octocrab::enterprises`].
pub struct EnterpriseHandler<'octo> {
    crab: &'octo Octocrab,
    enterprise: String,
}

impl<'octo> EnterpriseHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, enterprise: String) -> Self {
        Self { crab, enterprise }
    }

    /// Handle secret scanning alerts for the enterprise.
    ///
    /// See: https://docs.github.com/en/rest/secret-scanning?apiVersion=2022-11-28#list-secret-scanning-alerts-for-an-enterprise
    pub fn secret_scanning(&self) -> EnterpriseSecretScanningAlertsHandler<'_> {
        EnterpriseSecretScanningAlertsHandler::new(self)
    }

    /// Handle secret scanning alerts for the enterprise (alias for [`secret_scanning`][EnterpriseHandler::secret_scanning]).
    pub fn secrets_scanning(&self) -> EnterpriseSecretScanningAlertsHandler<'_> {
        self.secret_scanning()
    }
}

/// A client to GitHub's enterprise Secret Scanning API.
///
/// Created with [`EnterpriseHandler::secret_scanning`].
pub struct EnterpriseSecretScanningAlertsHandler<'octo> {
    handler: &'octo EnterpriseHandler<'octo>,
    params: Params,
}

impl<'octo> EnterpriseSecretScanningAlertsHandler<'octo> {
    pub(crate) fn new(enterprise: &'octo EnterpriseHandler<'octo>) -> Self {
        Self {
            handler: enterprise,
            params: Params::default(),
        }
    }

    /// Lists secret scanning alerts for eligible repositories in an enterprise.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let alerts = octocrab.enterprises("my-enterprise")
    ///     .secret_scanning()
    ///     .direction("asc")
    ///     .get_alerts()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_alerts(
        &self,
    ) -> crate::Result<crate::Page<crate::models::repos::secret_scanning_alert::SecretScanningAlert>>
    {
        let route = format!(
            "/enterprises/{}/secret-scanning/alerts",
            self.handler.enterprise
        );
        self.handler.crab.get(route, Some(&self.params)).await
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.params.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.params.page = Some(page.into());
        self
    }

    /// Filter Secret Scanning Alerts by state.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.params.state = Some(state.into());
        self
    }

    /// Filter Secret Scanning Alerts by resolution.
    pub fn resolution(mut self, resolution: impl Into<Vec<String>>) -> Self {
        self.params.resolution = Some(resolution.into());
        self
    }

    /// Filter Secret Scanning Alerts by validity.
    pub fn validity(mut self, validity: impl Into<Vec<String>>) -> Self {
        self.params.validity = Some(validity.into());
        self
    }

    /// Filter Secret Scanning Alerts by secret_type.
    pub fn secret_type(mut self, secret_type: impl Into<String>) -> Self {
        self.params.secret_type = Some(secret_type.into());
        self
    }

    /// Filter Secret Scanning Alerts by multi repo alerts.
    pub fn is_multi_repo(mut self, is_multi_repo: impl Into<bool>) -> Self {
        self.params.is_multi_repo = Some(is_multi_repo.into());
        self
    }

    /// Filter Secret Scanning Alerts by publicly leaked.
    pub fn is_publicly_leaked(mut self, is_publicly_leaked: impl Into<bool>) -> Self {
        self.params.is_publicly_leaked = Some(is_publicly_leaked.into());
        self
    }

    /// Hide secrets in results.
    pub fn hide_secret(mut self, hide_secret: impl Into<bool>) -> Self {
        self.params.hide_secret = Some(hide_secret.into());
        self
    }

    /// Filter Secret Scanning Alerts by push protection bypass status.
    pub fn is_bypassed(mut self, is_bypassed: impl Into<bool>) -> Self {
        self.params.is_bypassed = Some(is_bypassed.into());
        self
    }

    /// Filter Secret Scanning Alerts by after cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.params.after = Some(after.into());
        self
    }

    /// Filter Secret Scanning Alerts by before cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.params.before = Some(before.into());
        self
    }

    /// Sort Secret Scanning Alerts.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.params.sort = Some(sort.into());
        self
    }

    /// Sort direction of Secret Scanning Alerts.
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.params.direction = Some(direction.into());
        self
    }
}
