use super::RepoHandler;

/// A client to GitHub's repository Secret Scanning API.
///
/// Created with [`Octocrab::repos`](crate::Octocrab::repos).
pub struct RepoSecretScanningAlertsHandler<'octo> {
    handler: &'octo RepoHandler<'octo>,
    params: Params,
}

pub(crate) fn serialize_comma_separated<S>(
    val: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match val {
        Some(v) => serializer.serialize_str(&v.join(",")),
        None => serializer.serialize_none(),
    }
}

#[derive(serde::Serialize, Default, Clone)]
pub(crate) struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_comma_separated"
    )]
    pub resolution: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_comma_separated"
    )]
    pub validity: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_secret_types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_providers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_publicly_leaked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_repo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_secret: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bypassed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_email_hash: Option<String>,
}

impl<'octo> RepoSecretScanningAlertsHandler<'octo> {
    pub(crate) fn new(repo: &'octo RepoHandler<'octo>) -> Self {
        Self {
            handler: repo,
            params: Params::default(),
        }
    }

    /// Lists all Secret Scanning Alerts available in a repository.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let all_secrets = octocrab.repos("owner", "repo")
    ///     .secrets_scanning()
    ///     .direction("asc")
    ///     .get_alerts()
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_alerts(
        &self,
    ) -> crate::Result<crate::Page<crate::models::repos::secret_scanning_alert::SecretScanningAlert>>
    {
        let route = format!("/{}/secret-scanning/alerts", self.handler.repo);
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

    /// Lists single Secret Scanning Alert for a repository.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let all_secrets = octocrab.repos("owner", "repo")
    ///     .secrets_scanning()
    ///     .get_alert(5)
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_alert(
        &self,
        alert_number: u32,
    ) -> crate::Result<crate::models::repos::secret_scanning_alert::SecretScanningAlert> {
        let route = format!(
            "/{}/secret-scanning/alerts/{}",
            self.handler.repo, alert_number
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Updates a Secret Scanning alert.
    /// You must authenticate using an access token with the `security_events ` scope to use this endpoint.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::repos::secret_scanning_alert::UpdateSecretScanningAlert;
    ///
    /// let result = octocrab.repos("owner", "repo")
    ///     .secrets_scanning()
    ///     .update_alert(
    ///         5,
    ///         Some(&UpdateSecretScanningAlert {
    ///             state: "dismissed",
    ///             resolution: Some("no_bandwidth"),
    ///             resolution_comment: Some("I don't have time to fix this right now"),
    ///             ..Default::default()
    ///         })
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn update_alert(
        &self,
        alert_number: u32,
        alert_update: Option<
            &crate::models::repos::secret_scanning_alert::UpdateSecretScanningAlert<'_>,
        >,
    ) -> crate::Result<crate::models::repos::secret_scanning_alert::SecretScanningAlert> {
        let route = format!(
            "/{}/secret-scanning/alerts/{}",
            self.handler.repo, alert_number
        );
        self.handler.crab.patch(route, alert_update).await
    }

    // Get a Secret Scanning alert locations.
    /// You must authenticate using an access token with the `repo` or `security_events ` scope to use this endpoint.
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::repos::secret_scanning_alert::SecretsScanningAlertLocation;
    ///
    /// let result = octocrab.repos("owner", "repo")
    ///     .secrets_scanning()
    ///     .get_alert_locations(
    ///         5
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_alert_locations(
        &self,
        alert_number: u32,
    ) -> crate::Result<
        crate::Page<crate::models::repos::secret_scanning_alert::SecretsScanningAlertLocation>,
    > {
        let route = format!(
            "/{}/secret-scanning/alerts/{}/locations",
            self.handler.repo, alert_number
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}
