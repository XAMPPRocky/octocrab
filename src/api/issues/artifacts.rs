use super::IssueHandler;
use crate::models::issues::{IssueArtifact, IssueArtifactResponse};
use crate::Result;
use serde::Serialize;

/// Handler for GitHub's issue artifacts API.
///
/// Created with [`IssueHandler::artifacts`].
pub struct IssueArtifactsHandler<'octo, 'r> {
    handler: &'r IssueHandler<'octo>,
    issue_number: u64,
}

impl<'octo, 'r> IssueArtifactsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, issue_number: u64) -> Self {
        Self {
            handler,
            issue_number,
        }
    }

    /// List issue artifacts.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/issues/artifacts#list-issue-artifacts)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let artifacts = octocrab.issues("owner", "repo")
    ///     .artifacts(1)
    ///     .list()
    ///     .per_page(100)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListIssueArtifactsBuilder<'octo, 'r> {
        ListIssueArtifactsBuilder::new(self.handler, self.issue_number)
    }

    /// Create an issue artifact.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/issues/artifacts#create-an-issue-artifact)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let artifact = octocrab.issues("owner", "repo")
    ///     .artifacts(1)
    ///     .create("my-artifact", "Artifact content")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        name: impl Into<String>,
        body: impl Into<String>,
    ) -> CreateIssueArtifactBuilder<'octo, 'r> {
        CreateIssueArtifactBuilder::new(self.handler, self.issue_number, name.into(), body.into())
    }

    /// Get an issue artifact.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/issues/artifacts#get-an-issue-artifact)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let artifact = octocrab.issues("owner", "repo")
    ///     .artifacts(1)
    ///     .get(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, artifact_number: u64) -> Result<IssueArtifactResponse> {
        let route = format!(
            "/{}/issues/{}/artifacts/{}",
            self.handler.repo, self.issue_number, artifact_number,
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update an issue artifact.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/issues/artifacts#update-an-issue-artifact)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let artifact = octocrab.issues("owner", "repo")
    ///     .artifacts(1)
    ///     .update(1)
    ///     .body("Updated content")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, artifact_number: u64) -> UpdateIssueArtifactBuilder<'octo, 'r> {
        UpdateIssueArtifactBuilder::new(self.handler, self.issue_number, artifact_number)
    }

    /// Delete an issue artifact.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/issues/artifacts#delete-an-issue-artifact)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.issues("owner", "repo")
    ///     .artifacts(1)
    ///     .delete(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, artifact_number: u64) -> Result<()> {
        let route = format!(
            "/{}/issues/{}/artifacts/{}",
            self.handler.repo, self.issue_number, artifact_number,
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

#[derive(Serialize)]
pub struct ListIssueArtifactsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    issue_number: u64,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    artifact_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListIssueArtifactsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, issue_number: u64) -> Self {
        Self {
            handler,
            issue_number,
            artifact_type: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter artifacts by type (`generic`, `link`, `plan`, `embedded_html`).
    pub fn artifact_type(mut self, artifact_type: impl Into<String>) -> Self {
        self.artifact_type = Some(artifact_type.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> Result<crate::Page<IssueArtifact>> {
        let route = format!(
            "/{}/issues/{}/artifacts",
            self.handler.repo, self.issue_number,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(Serialize)]
pub struct CreateIssueArtifactBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    issue_number: u64,
    name: String,
    body: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    artifact_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl<'octo, 'r> CreateIssueArtifactBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r IssueHandler<'octo>,
        issue_number: u64,
        name: String,
        body: String,
    ) -> Self {
        Self {
            handler,
            issue_number,
            name,
            body,
            artifact_type: None,
            description: None,
        }
    }

    pub fn artifact_type(mut self, artifact_type: impl Into<String>) -> Self {
        self.artifact_type = Some(artifact_type.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub async fn send(self) -> Result<IssueArtifact> {
        let route = format!(
            "/{}/issues/{}/artifacts",
            self.handler.repo, self.issue_number,
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}

#[derive(Serialize)]
pub struct UpdateIssueArtifactBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    issue_number: u64,
    #[serde(skip)]
    artifact_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl<'octo, 'r> UpdateIssueArtifactBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r IssueHandler<'octo>,
        issue_number: u64,
        artifact_number: u64,
    ) -> Self {
        Self {
            handler,
            issue_number,
            artifact_number,
            name: None,
            body: None,
            description: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub async fn send(self) -> Result<IssueArtifact> {
        let route = format!(
            "/{}/issues/{}/artifacts/{}",
            self.handler.repo, self.issue_number, self.artifact_number,
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}
