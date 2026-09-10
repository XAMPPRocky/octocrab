pub use crate::api::repos::security_advisories::{
    ListRepoSecurityAdvisoriesBuilder, RepoSecurityAdvisoriesHandler,
};
use crate::models::security_advisories::{RepositoryAdvisory, SecurityAdvisory};
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's Security Advisories API.
///
/// Created with [`Octocrab::security_advisories`].
pub struct SecurityAdvisoriesHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> SecurityAdvisoriesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// List global security advisories.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/global-advisories#list-global-security-advisories
    pub fn list_global_advisories(&self) -> ListGlobalSecurityAdvisoriesBuilder<'octo, '_> {
        ListGlobalSecurityAdvisoriesBuilder::new(self)
    }

    /// Shortcut for [`Self::list_global_advisories`].
    pub fn list(&self) -> ListGlobalSecurityAdvisoriesBuilder<'octo, '_> {
        self.list_global_advisories()
    }

    /// Get a global security advisory using its GHSA identifier.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/global-advisories#get-a-global-security-advisory
    pub async fn get_global_advisory(&self, ghsa_id: impl AsRef<str>) -> Result<SecurityAdvisory> {
        let route = format!("/advisories/{ghsa_id}", ghsa_id = ghsa_id.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Shortcut for [`Self::get_global_advisory`].
    pub async fn get(&self, ghsa_id: impl AsRef<str>) -> Result<SecurityAdvisory> {
        self.get_global_advisory(ghsa_id).await
    }

    /// Access repository security advisories for an organization.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#list-repository-security-advisories-for-an-organization
    pub fn org(&self, org: impl Into<String>) -> OrgSecurityAdvisoriesHandler<'octo> {
        OrgSecurityAdvisoriesHandler::new(self.crab, org.into())
    }
}

/// A builder pattern struct for listing global security advisories.
#[derive(serde::Serialize)]
pub struct ListGlobalSecurityAdvisoriesBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b SecurityAdvisoriesHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ghsa_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cve_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecosystem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_withdrawn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affects: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epss_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epss_percentile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
}

impl<'octo, 'b> ListGlobalSecurityAdvisoriesBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b SecurityAdvisoriesHandler<'octo>) -> Self {
        Self {
            handler,
            ghsa_id: None,
            r#type: None,
            cve_id: None,
            ecosystem: None,
            severity: None,
            cwes: None,
            is_withdrawn: None,
            affects: None,
            published: None,
            updated: None,
            modified: None,
            epss_percentage: None,
            epss_percentile: None,
            before: None,
            after: None,
            direction: None,
            per_page: None,
            sort: None,
        }
    }

    pub fn ghsa_id(mut self, ghsa_id: impl Into<String>) -> Self {
        self.ghsa_id = Some(ghsa_id.into());
        self
    }

    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    pub fn cve_id(mut self, cve_id: impl Into<String>) -> Self {
        self.cve_id = Some(cve_id.into());
        self
    }

    pub fn ecosystem(mut self, ecosystem: impl Into<String>) -> Self {
        self.ecosystem = Some(ecosystem.into());
        self
    }

    pub fn severity(mut self, severity: impl Into<String>) -> Self {
        self.severity = Some(severity.into());
        self
    }

    pub fn cwes(mut self, cwes: impl Into<String>) -> Self {
        self.cwes = Some(cwes.into());
        self
    }

    pub fn is_withdrawn(mut self, is_withdrawn: bool) -> Self {
        self.is_withdrawn = Some(is_withdrawn);
        self
    }

    pub fn affects(mut self, affects: impl Into<String>) -> Self {
        self.affects = Some(affects.into());
        self
    }

    pub fn published(mut self, published: impl Into<String>) -> Self {
        self.published = Some(published.into());
        self
    }

    pub fn updated(mut self, updated: impl Into<String>) -> Self {
        self.updated = Some(updated.into());
        self
    }

    pub fn modified(mut self, modified: impl Into<String>) -> Self {
        self.modified = Some(modified.into());
        self
    }

    pub fn epss_percentage(mut self, epss_percentage: impl Into<String>) -> Self {
        self.epss_percentage = Some(epss_percentage.into());
        self
    }

    pub fn epss_percentile(mut self, epss_percentile: impl Into<String>) -> Self {
        self.epss_percentile = Some(epss_percentile.into());
        self
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub async fn send(&self) -> Result<Page<SecurityAdvisory>> {
        self.handler.crab.get("/advisories", Some(&self)).await
    }
}

/// A handler and builder for listing repository security advisories for an organization.
#[derive(serde::Serialize)]
pub struct OrgSecurityAdvisoriesHandler<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    org: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl<'octo> OrgSecurityAdvisoriesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String) -> Self {
        Self {
            crab,
            org,
            direction: None,
            sort: None,
            before: None,
            after: None,
            per_page: None,
            state: None,
        }
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub async fn list(&self) -> Result<Page<RepositoryAdvisory>> {
        self.send().await
    }

    pub async fn send(&self) -> Result<Page<RepositoryAdvisory>> {
        let route = format!("/orgs/{org}/security-advisories", org = self.org);
        self.crab.get(route, Some(&self)).await
    }
}
