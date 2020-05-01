use crate::{models, Octocrab, Result};

pub mod create;

pub struct IssueHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> IssueHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// Adds up to 10 assignees to an issue. Users already assigned to an issue
    /// are not replaced.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.issues("owner", "repo").add_assignees(101, &[56982]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_assignees(&self, number: u64, assignees: &[u64]) -> Result<models::Issue> {
        let route = format!(
            "/repos/{owner}/{repo}/issues/{issue}/assignees",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(
                self.crab.absolute_url(route)?,
                Some(&serde_json::json!({ "assignees": assignees })),
            )
            .await
    }

    /// Adds `labels` to an issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.issues("owner", "repo").add_labels(101, &[String::from("help wanted")]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_labels(&self, number: u64, labels: &[String]) -> Result<models::Label> {
        let route = format!(
            "/repos/{owner}/{repo}/issues/{issue}/labels",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(
                self.crab.absolute_url(route)?,
                Some(&serde_json::json!({ "labels": labels })),
            )
            .await
    }

    /// Checks if a user has permission to be assigned to an issue in
    /// this repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.issues("owner", "repo").check_assignee("ferris").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_assignee(&self, assignee: impl AsRef<str>) -> Result<bool> {
        let route = format!(
            "/repos/{owner}/{repo}/assignees/{assignee}",
            owner = self.owner,
            repo = self.repo,
            assignee = assignee.as_ref()
        );
        let response = self
            .crab
            ._get(self.crab.absolute_url(route)?, None::<&()>)
            .await?;
        let status = response.status();

        if status == 204 {
            Ok(true)
        } else if status == 404 {
            Ok(false)
        } else {
            Err(Octocrab::map_github_error(response).await.unwrap_err())
        }
    }

    /// Checks if a user has permission to be assigned to an issue in
    /// this repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.issues("owner", "repo").create("My first issue").send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, title: impl Into<String>) -> create::CreateIssueBuilder<'_, '_> {
        create::CreateIssueBuilder::new(self, title.into())
    }

    /// Creates a comment in the issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.issues("owner", "repo").create_comment(101, "Beep Boop").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_comment(
        &self,
        number: u64,
        body: impl AsRef<str>,
    ) -> Result<models::Comment> {
        let route = format!(
            "/repos/{owner}/{repo}/issues/{issue}/comments",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(
                self.crab.absolute_url(route)?,
                Some(&serde_json::json!({ "body": body.as_ref() })),
            )
            .await
    }

    /// Creates a label in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.issues("owner", "repo").create_label("help wanted", "59dd5a", "").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_label(
        &self,
        name: impl AsRef<str>,
        color: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<models::Label> {
        let route = format!(
            "/repos/{owner}/{repo}/labels",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab
            .post(
                self.crab.absolute_url(route)?,
                Some(&serde_json::json!({
                    "name": name.as_ref(),
                    "color": color.as_ref(),
                    "description": description.as_ref()
                })),
            )
            .await
    }
}
