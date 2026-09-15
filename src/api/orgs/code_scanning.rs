use super::OrgHandler;
use crate::models::code_scannings::CodeScanningAlert;
use crate::params::{self, Direction};
use crate::{Page, Result};

/// A client to GitHub's organization Code Scanning API.
///
/// Created with [`OrgHandler::code_scanning`].
pub struct OrgCodeScanningHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgCodeScanningHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List code scanning alerts for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-alerts-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .orgs("owner")
    ///     .code_scanning()
    ///     .list()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListOrgCodeScanningAlertsBuilder<'octo, 'b, '_> {
        ListOrgCodeScanningAlertsBuilder::new(self)
    }

    /// Alias for [`Self::list`].
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-alerts-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .orgs("owner")
    ///     .code_scanning()
    ///     .list_alerts()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_alerts(&self) -> ListOrgCodeScanningAlertsBuilder<'octo, 'b, '_> {
        self.list()
    }
}

/// Builder for listing organization code scanning alerts.
#[derive(serde::Serialize)]
pub struct ListOrgCodeScanningAlertsBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c OrgCodeScanningHandler<'octo, 'b>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<params::code_scannings::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<params::code_scannings::Severity>,
}

impl<'octo, 'b, 'c> ListOrgCodeScanningAlertsBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c OrgCodeScanningHandler<'octo, 'b>) -> Self {
        Self {
            handler,
            tool_name: None,
            tool_guid: None,
            before: None,
            after: None,
            page: None,
            per_page: None,
            direction: None,
            state: None,
            sort: None,
            severity: None,
        }
    }

    /// Filter by the name of the tool used to generate the code scanning analysis.
    pub fn tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.tool_name = Some(tool_name.into());
        self
    }

    /// Filter by the GUID of the tool used to generate the code scanning analysis.
    pub fn tool_guid(mut self, tool_guid: impl Into<String>) -> Self {
        self.tool_guid = Some(tool_guid.into());
        self
    }

    /// Cursor for pagination (before).
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Cursor for pagination (after).
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// The direction to sort the results by.
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Filter by the state of the code scanning alert.
    pub fn state(mut self, state: impl Into<params::State>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// The property to sort the results by.
    pub fn sort(mut self, sort: impl Into<params::code_scannings::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Filter by the severity of the code scanning alert.
    pub fn severity(mut self, severity: impl Into<params::code_scannings::Severity>) -> Self {
        self.severity = Some(severity.into());
        self
    }

    /// Sends the request to list code scanning alerts.
    pub async fn send(self) -> Result<Page<CodeScanningAlert>> {
        let route = format!("/orgs/{}/code-scanning/alerts", self.handler.handler.owner);
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
