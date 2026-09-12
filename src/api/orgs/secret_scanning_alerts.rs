use super::OrgHandler;
use crate::api::repos::secret_scanning_alerts::Params;

/// A client to GitHub's organization Secret Scanning API.
///
/// Created with [`OrgHandler::secret_scanning`](crate::api::orgs::OrgHandler::secret_scanning).
pub struct OrgSecretScanningAlertsHandler<'octo> {
    handler: &'octo OrgHandler<'octo>,
    params: Params,
}

impl<'octo> OrgSecretScanningAlertsHandler<'octo> {
    pub(crate) fn new(org: &'octo OrgHandler<'octo>) -> Self {
        Self {
            handler: org,
            params: Params::default(),
        }
    }

    /// Lists secret scanning alerts for eligible repositories in an organization.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab.orgs("org")
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
        let route = format!("/orgs/{}/secret-scanning/alerts", self.handler.owner);
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

    /// Exclude Secret Scanning Alerts by secret_type.
    pub fn exclude_secret_types(mut self, exclude_secret_types: impl Into<String>) -> Self {
        self.params.exclude_secret_types = Some(exclude_secret_types.into());
        self
    }

    /// Filter Secret Scanning Alerts by provider slug.
    pub fn providers(mut self, providers: impl Into<String>) -> Self {
        self.params.providers = Some(providers.into());
        self
    }

    /// Exclude Secret Scanning Alerts by provider slug.
    pub fn exclude_providers(mut self, exclude_providers: impl Into<String>) -> Self {
        self.params.exclude_providers = Some(exclude_providers.into());
        self
    }

    /// Filter Secret Scanning Alerts by assignee.
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.params.assignee = Some(assignee.into());
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

    /// Filter Secret Scanning Alerts by attached metadata fields.
    pub fn included_metadata(mut self, included_metadata: impl Into<String>) -> Self {
        self.params.included_metadata = Some(included_metadata.into());
        self
    }

    /// Filter Secret Scanning Alerts by attached owner_email metadata hash.
    pub fn owner_email_hash(mut self, owner_email_hash: impl Into<String>) -> Self {
        self.params.owner_email_hash = Some(owner_email_hash.into());
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
