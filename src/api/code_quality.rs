//! The code quality API.
use http::{StatusCode, Uri};
use snafu::ResultExt;

use crate::error::HttpSnafu;
use crate::models::code_quality::{
    CodeQualityEnablement, CodeQualityFinding, CodeQualitySetup, UpdateCodeQualitySetup,
    UpdateCodeQualitySetupResponse,
};
use crate::params::Direction;
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's code quality API.
///
/// Created with [`Octocrab::code_quality`] or [`Octocrab::code_quality_org`].
pub struct CodeQualityHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: Option<String>,
}

impl<'octo> CodeQualityHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: Option<String>) -> Self {
        Self { crab, owner, repo }
    }

    /// List code quality findings for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-quality/code-quality?apiVersion=2022-11-28#list-code-quality-findings-for-a-repository)
    pub fn list_findings(&self) -> ListCodeQualityFindingsBuilder<'octo, '_> {
        ListCodeQualityFindingsBuilder::new(self)
    }

    /// Get a code quality finding for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-quality/code-quality?apiVersion=2022-11-28#get-a-code-quality-finding)
    pub async fn get_finding(&self, finding_number: u64) -> Result<CodeQualityFinding> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-quality/findings/{}",
            self.owner, repo, finding_number
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get a code quality setup configuration for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-quality/code-quality?apiVersion=2022-11-28#get-a-code-quality-setup-configuration)
    pub async fn get_setup(&self) -> Result<CodeQualitySetup> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-quality/setup", self.owner, repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Update a code quality setup configuration for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-quality/code-quality?apiVersion=2022-11-28#update-a-code-quality-setup-configuration)
    pub async fn update_setup(
        &self,
        body: &UpdateCodeQualitySetup,
    ) -> Result<UpdateCodeQualitySetupResponse> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-quality/setup", self.owner, repo);
        self.crab.patch(route, Some(body)).await
    }

    /// Enable or disable code quality for an organization.
    pub async fn set_org_enablement(&self, body: &CodeQualityEnablement) -> Result<()> {
        let route = format!("/orgs/{}/code-quality/enablement", self.owner);
        let response = self.crab._post(route, Some(body)).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Enable code quality for a repository.
    pub async fn enable(&self) -> Result<()> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-quality", self.owner, repo);
        let response = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Disable code quality for a repository.
    pub async fn disable(&self) -> Result<()> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-quality", self.owner, repo);
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Check if code quality is enabled for a repository.
    pub async fn check(&self) -> Result<bool> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-quality/check", self.owner, repo);
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let response = self.crab._get(uri).await?;
        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(crate::map_github_error(response).await.unwrap_err()),
        }
    }
}

/// Builder for listing code quality findings for a repository.
#[derive(serde::Serialize)]
pub struct ListCodeQualityFindingsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b CodeQualityHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl<'octo, 'b> ListCodeQualityFindingsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b CodeQualityHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            direction: None,
            before: None,
            after: None,
            state: None,
        }
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

    /// Filter by state (`open` or `dismissed`).
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Sends the request to list code quality findings.
    pub async fn send(self) -> Result<Page<CodeQualityFinding>> {
        let repo = self
            .handler
            .repo
            .as_ref()
            .expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-quality/findings",
            self.handler.owner, repo
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
