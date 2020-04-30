use crate::{models, Octocrab};

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
    pub async fn add_assignees(&self, number: u64, assignees: &[u64]) -> crate::Result<models::Issue> {
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
    pub async fn add_labels(&self, number: u64, labels: &[String]) -> crate::Result<models::Label> {
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
}
