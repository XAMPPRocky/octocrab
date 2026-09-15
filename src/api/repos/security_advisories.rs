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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#list-repository-security-advisories)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let advisories = octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .list()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListRepoSecurityAdvisoriesBuilder<'octo, 'b, '_> {
        ListRepoSecurityAdvisoriesBuilder::new(self)
    }

    /// Get a repository security advisory by its GHSA ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#get-a-repository-security-advisory)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let advisory = octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .get("GHSA-xxxx-xxxx-xxxx")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#create-a-repository-security-advisory)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::security_advisories::CreateRepositoryAdvisory;
    ///
    /// # let body: CreateRepositoryAdvisory = todo!();
    /// let advisory = octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .create(&body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, body: &CreateRepositoryAdvisory) -> Result<RepositoryAdvisory> {
        let route = format!("/{}/security-advisories", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Update a repository security advisory.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#update-a-repository-security-advisory)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::security_advisories::UpdateRepositoryAdvisory;
    ///
    /// # let body: UpdateRepositoryAdvisory = todo!();
    /// let advisory = octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .update("GHSA-xxxx-xxxx-xxxx", &body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#privately-report-a-security-vulnerability)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::security_advisories::ReportVulnerability;
    ///
    /// # let body: ReportVulnerability = todo!();
    /// let advisory = octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .report(&body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn report(&self, body: &ReportVulnerability) -> Result<RepositoryAdvisory> {
        let route = format!("/{}/security-advisories/reports", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Request a CVE identification number for a repository security advisory.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#request-a-cve-for-a-repository-security-advisory)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .request_cve("GHSA-xxxx-xxxx-xxxx")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/security-advisories/repository-advisories?apiVersion=2022-11-28#create-a-temporary-private-fork)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let fork = octocrab
    ///     .repos("owner", "repo")
    ///     .security_advisories()
    ///     .create_fork("GHSA-xxxx-xxxx-xxxx")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// The direction to sort the results by.
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// The property to sort the results by.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// A cursor, as given in the Link header, used for pagination.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// A cursor, as given in the Link header, used for pagination.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// The number of results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Filter by the state of the repository advisories (e.g. "triage", "draft", "published", "closed").
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Sends the request to list security advisories in a repository.
    pub async fn send(&self) -> Result<Page<RepositoryAdvisory>> {
        let route = format!("/{}/security-advisories", self.handler.handler.repo);
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
