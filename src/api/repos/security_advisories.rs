use super::RepoHandler;
use crate::models::security_advisories::{
    CreateRepositoryAdvisory, ReportVulnerability, RepositoryAdvisory, UpdateRepositoryAdvisory,
};
use crate::models::Repository;
use crate::{FromResponse, Page, Result};

/// A client to GitHub's repository security advisories API.
///
/// Created with [`RepoHandler::security_advisories`].
pub struct RepoSecurityAdvisoriesHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoSecurityAdvisoriesHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List security advisories in a repository.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#list-repository-security-advisories
    pub fn list(&self) -> ListRepoSecurityAdvisoriesBuilder<'octo, 'b, '_> {
        ListRepoSecurityAdvisoriesBuilder::new(self)
    }

    /// Get a repository security advisory by its GHSA ID.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#get-a-repository-security-advisory
    pub async fn get(&self, ghsa_id: impl AsRef<str>) -> Result<RepositoryAdvisory> {
        let route = format!(
            "/{}/security-advisories/{}",
            self.handler.repo,
            ghsa_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create a new repository security advisory.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#create-a-repository-security-advisory
    pub async fn create(&self, body: &CreateRepositoryAdvisory) -> Result<RepositoryAdvisory> {
        let route = format!("/{}/security-advisories", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Update a repository security advisory.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#update-a-repository-security-advisory
    pub async fn update(
        &self,
        ghsa_id: impl AsRef<str>,
        body: &UpdateRepositoryAdvisory,
    ) -> Result<RepositoryAdvisory> {
        let route = format!(
            "/{}/security-advisories/{}",
            self.handler.repo,
            ghsa_id.as_ref()
        );
        self.handler.crab.patch(route, Some(body)).await
    }

    /// Privately report a security vulnerability.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#privately-report-a-security-vulnerability
    pub async fn report(&self, body: &ReportVulnerability) -> Result<RepositoryAdvisory> {
        let route = format!("/{}/security-advisories/reports", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Request a CVE identification number for a repository security advisory.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#request-a-cve-for-a-repository-security-advisory
    pub async fn request_cve(&self, ghsa_id: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/{}/security-advisories/{}/cve",
            self.handler.repo,
            ghsa_id.as_ref()
        );
        let uri = self.handler.crab.parameterized_uri(route, None::<&()>)?;
        let response = self.handler.crab._post(uri, None::<&()>).await?;
        if response.status() != http::StatusCode::ACCEPTED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Create a temporary private fork to collaborate on fixing a security vulnerability.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#create-a-temporary-private-fork
    pub async fn create_fork(&self, ghsa_id: impl AsRef<str>) -> Result<Repository> {
        let route = format!(
            "/{}/security-advisories/{}/forks",
            self.handler.repo,
            ghsa_id.as_ref()
        );
        let uri = self.handler.crab.parameterized_uri(route, None::<&()>)?;
        let response = self.handler.crab._post(uri, None::<&()>).await?;
        if response.status() != http::StatusCode::ACCEPTED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        <Repository>::from_response(crate::map_github_error(response).await?).await
    }
}

#[derive(serde::Serialize)]
pub struct ListRepoSecurityAdvisoriesBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c RepoSecurityAdvisoriesHandler<'octo, 'b>,
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

impl<'octo, 'b, 'c> ListRepoSecurityAdvisoriesBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c RepoSecurityAdvisoriesHandler<'octo, 'b>) -> Self {
        Self {
            handler,
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

    pub async fn send(&self) -> Result<Page<RepositoryAdvisory>> {
        let route = format!("/{}/security-advisories", self.handler.handler.repo);
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
