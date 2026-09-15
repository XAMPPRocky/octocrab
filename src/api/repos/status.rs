use super::*;
use crate::models::{Status, StatusState};

#[derive(serde::Serialize)]
pub struct CreateStatusBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    pub sha: String,
    pub state: StatusState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl<'octo, 'r> CreateStatusBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, sha: String, state: StatusState) -> Self {
        Self {
            handler,
            sha,
            state,
            context: None,
            target_url: None,
            description: None,
        }
    }

    /// The SHA hash of the target commit.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::StatusState;
    ///
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .create_status("sha".to_string(), StatusState::Success)
    ///     .sha("other_sha".to_string())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sha(mut self, sha: String) -> Self {
        self.sha = sha;
        self
    }

    /// A string label to differentiate this status from the status of other systems.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::StatusState;
    ///
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .create_status("sha".to_string(), StatusState::Success)
    ///     .context("continuous-integration/jenkins".to_string())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    /// A short description of the status.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::StatusState;
    ///
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .create_status("sha".to_string(), StatusState::Success)
    ///     .description("Build succeeded".to_string())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// The target URL to associate with this status. This URL will be linked from the GitHub UI to allow users to easily see the source of the status.
    /// For example, if your continuous integration system is posting build status, you would want to provide the deep link for the build output for this specific SHA:
    /// <http://ci.example.com/user/repo/build/sha>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::StatusState;
    ///
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .create_status("sha".to_string(), StatusState::Success)
    ///     .target("https://ci.example.com/build/1".to_string())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn target(mut self, target: String) -> Self {
        self.target_url = Some(target);
        self
    }

    /// The state of the status.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::StatusState;
    ///
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .create_status("sha".to_string(), StatusState::Pending)
    ///     .state(StatusState::Success)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn state(mut self, state: StatusState) -> Self {
        self.state = state;
        self
    }

    /// Sends the actual request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::StatusState;
    ///
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .create_status("sha".to_string(), StatusState::Success)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Status> {
        let route = format!("/{}/statuses/{sha}", self.handler.repo, sha = self.sha);
        self.handler.crab.post(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListStatusesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListStatusesBuilder<'octo, 'r> {
    /// Creates a new [`ListStatusesBuilder`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let repo = octocrab.repos("owner", "repo");
    /// let builder = octocrab::repos::ListStatusesBuilder::new(&repo, "sha".to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(handler: &'r RepoHandler<'octo>, sha: String) -> Self {
        Self {
            handler,
            sha,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let statuses = octocrab
    ///     .repos("owner", "repo")
    ///     .list_statuses("sha".to_string())
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let statuses = octocrab
    ///     .repos("owner", "repo")
    ///     .list_statuses("sha".to_string())
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let statuses = octocrab
    ///     .repos("owner", "repo")
    ///     .list_statuses("sha".to_string())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Status>> {
        let route = format!(
            "/{repo}/commits/{sha}/statuses",
            repo = self.handler.repo,
            sha = self.sha,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
