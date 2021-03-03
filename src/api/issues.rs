//! The issue API.

mod create;
mod list;
mod list_labels;
mod update;

use crate::{models, params, Octocrab, Result};

pub use self::{
    create::CreateIssueBuilder,
    list::ListIssuesBuilder,
    list_labels::{ListLabelsForIssueBuilder, ListLabelsForRepoBuilder},
    update::UpdateIssueBuilder,
};

/// Handler for GitHub's issue API.
///
/// Note: GitHub's REST API v3 considers every pull request an issue, but not
/// every issue is a pull request. For this reason, "Issues" endpoints may
/// return both issues and pull requests in the response. You can identify pull
/// requests by the `pull_request` key.
///
/// Created with [`Octocrab::issues`].
pub struct IssueHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> IssueHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// Gets a label from the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let label = octocrab.issues("owner", "repo").get_label("help wanted").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, number: u64) -> Result<models::issues::Issue> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{number}",
            owner = self.owner,
            repo = self.repo,
            number = number,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Create a issue in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let issue = octocrab.issues("owner", "repo").create("My first issue")
    ///     // Optional Parameters
    ///     .body("This is an autogenerated issue..")
    ///     .milestone(1001)
    ///     .labels(vec![String::from("help-wanted")])
    ///     .assignees(vec![String::from("ferris")])
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, title: impl Into<String>) -> create::CreateIssueBuilder<'_, '_> {
        create::CreateIssueBuilder::new(self, title.into())
    }

    /// List issues in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let issue = octocrab.issues("owner", "repo")
    ///     .list()
    ///     // Optional Parameters
    ///     .state(params::State::All)
    ///     .milestone(1234)
    ///     .assignee("ferris")
    ///     .creator("octocrab")
    ///     .mentioned("octocat")
    ///     .labels(&[String::from("help wanted"), String::from("good first issue")])
    ///     .sort(params::issues::Sort::Comments)
    ///     .direction(params::Direction::Ascending)
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> list::ListIssuesBuilder<'_, '_, '_, '_> {
        list::ListIssuesBuilder::new(self)
    }

    /// Update an issue in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models;
    ///
    /// let issue = octocrab.issues("owner", "repo")
    ///     .update(1234u64)
    ///     // Optional Parameters
    ///     .title("Updated title")
    ///     .body("New body")
    ///     .state(models::IssueState::Closed)
    ///     .milestone(1234u64)
    ///     .assignees(&[String::from("ferris")])
    ///     .labels(&[String::from("help wanted"), String::from("good first issue")])
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, number: u64) -> update::UpdateIssueBuilder<'_, '_, '_, '_, '_, '_> {
        update::UpdateIssueBuilder::new(self, number)
    }

    /// Users with push access can lock an issue or pull request's conversation.
    ///
    /// *Note* Providing a reason requires the `sailor-v` preview to enabled.
    ///
    /// See also: https://developer.github.com/v3/issues/#lock-an-issue
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// assert!(octocrab::instance().issues("owner", "repo").lock(404, params::LockReason::OffTopic).await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn lock(
        &self,
        number: u64,
        reason: impl Into<Option<params::LockReason>>,
    ) -> Result<bool> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{number}/lock",
            owner = self.owner,
            repo = self.repo,
            number = number,
        );

        let response = self
            .crab
            ._put(
                self.crab.absolute_url(route)?,
                reason
                    .into()
                    .map(|reason| {
                        serde_json::json!({
                            "lock_reason": reason,
                        })
                    })
                    .as_ref(),
            )
            .await?;

        Ok(response.status() == 204)
    }

    /// Users with push access can unlock an issue or pull request's conversation.
    ///
    /// See also: https://developer.github.com/v3/issues/#unlock-an-issue
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// assert!(octocrab::instance().issues("owner", "repo").unlock(404).await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unlock(&self, number: u64) -> Result<bool> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{number}/lock",
            owner = self.owner,
            repo = self.repo,
            number = number,
        );

        let response = self
            .crab
            ._delete(self.crab.absolute_url(route)?, None::<&()>)
            .await?;

        Ok(response.status() == 204)
    }
}

/// # Assignees
impl<'octo> IssueHandler<'octo> {
    /// Adds up to 10 assignees to an issue. Users already assigned to an issue
    /// are not replaced.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let issue = octocrab.issues("owner", "repo").add_assignees(101, &[56982]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_assignees(
        &self,
        number: u64,
        assignees: &[u64],
    ) -> Result<models::issues::Issue> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue}/assignees",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(route, Some(&serde_json::json!({ "assignees": assignees })))
            .await
    }

    /// Checks if a user has permission to be assigned to an issue in
    /// the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// assert!(octocrab.issues("owner", "repo").check_assignee("ferris").await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_assignee(&self, assignee: impl AsRef<str>) -> Result<bool> {
        let route = format!(
            "repos/{owner}/{repo}/assignees/{assignee}",
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
            Err(crate::map_github_error(response).await.unwrap_err())
        }
    }

    /// Lists the available assignees for issues in a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let assignees = octocrab
    ///     .issues("owner", "repo")
    ///     .list_assignees()
    ///     .per_page(15)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_assignees(&self) -> ListAssigneesBuilder {
        ListAssigneesBuilder::new(self)
    }
}

#[derive(serde::Serialize)]
pub struct ListAssigneesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListAssigneesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::User>> {
        let route = format!(
            "repos/{owner}/{repo}/assignees",
            owner = self.handler.owner,
            repo = self.handler.repo,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

/// # Labels
impl<'octo> IssueHandler<'octo> {
    /// Adds `labels` to an issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let labels = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .add_labels(101, &[String::from("help wanted")])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_labels(&self, number: u64, labels: &[String]) -> Result<Vec<models::Label>> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue}/labels",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(route, Some(&serde_json::json!({ "labels": labels })))
            .await
    }

    /// Removes `label` from an issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let removed_labels = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .remove_label(101, "my_label")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_label(
        &self,
        number: u64,
        label: impl AsRef<str>,
    ) -> Result<Vec<models::Label>> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue_number}/labels/{name}",
            owner = self.owner,
            repo = self.repo,
            issue_number = number,
            name = label.as_ref(),
        );

        self.crab.delete(route, None::<&()>).await
    }

    /// Replaces all labels for an issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let labels = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .replace_all_labels(101, &[String::from("help wanted")])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn replace_all_labels(
        &self,
        number: u64,
        labels: &[String],
    ) -> Result<Vec<models::Label>> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue}/labels",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .put(route, Some(&serde_json::json!({ "labels": labels })))
            .await
    }

    /// Creates a label in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let label = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .create_label("help wanted", "59dd5a", "")
    ///     .await?;
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
            "repos/{owner}/{repo}/labels",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab
            .post(
                route,
                Some(&serde_json::json!({
                    "name": name.as_ref(),
                    "color": color.as_ref(),
                    "description": description.as_ref()
                })),
            )
            .await
    }

    /// Gets a label from the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let label = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .get_label("help wanted")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_label(&self, name: impl AsRef<str>) -> Result<models::Label> {
        let route = format!(
            "repos/{owner}/{repo}/labels/{name}",
            owner = self.owner,
            repo = self.repo,
            name = name.as_ref(),
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Deletes a label in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let label = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .delete_label("help wanted")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_label(&self, name: impl AsRef<str>) -> Result<models::Label> {
        let route = format!(
            "repos/{owner}/{repo}/labels/{name}",
            owner = self.owner,
            repo = self.repo,
            name = name.as_ref(),
        );

        self.crab.delete(route, None::<&()>).await
    }

    /// List labels from an issue on a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .list_labels_for_issue(404)
    ///     // Optional Parameters
    ///     .per_page(20)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_labels_for_issue(&self, number: u64) -> ListLabelsForIssueBuilder {
        ListLabelsForIssueBuilder::new(self, number)
    }

    /// List all labels from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .list_labels_for_repo()
    ///     // Optional Parameters
    ///     .per_page(20)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_labels_for_repo(&self) -> ListLabelsForRepoBuilder {
        ListLabelsForRepoBuilder::new(self)
    }
}

/// # Comments
impl<'octo> IssueHandler<'octo> {
    /// Creates a comment in the issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .create_comment(101, "Beep Boop")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_comment(
        &self,
        number: u64,
        body: impl AsRef<str>,
    ) -> Result<models::issues::Comment> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue}/comments",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(route, Some(&serde_json::json!({ "body": body.as_ref() })))
            .await
    }

    /// Gets a comment in the issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .get_comment(101)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_comment(&self, comment_id: u64) -> Result<models::issues::Comment> {
        let route = format!(
            "repos/{owner}/{repo}/issues/comments/{comment_id}",
            owner = self.owner,
            repo = self.repo,
            comment_id = comment_id
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Updates a comment in the issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .update_comment(101, "Beep Boop")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_comment(
        &self,
        comment_id: u64,
        body: impl AsRef<str>,
    ) -> Result<models::issues::Comment> {
        let route = format!(
            "repos/{owner}/{repo}/issues/comments/{comment_id}",
            owner = self.owner,
            repo = self.repo,
            comment_id = comment_id
        );

        self.crab
            .post(route, Some(&serde_json::json!({ "body": body.as_ref() })))
            .await
    }

    /// Deletes a comment in an issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance().issues("owner", "repo").delete_comment(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_comment(&self, comment_id: u64) -> Result<()> {
        let route = format!(
            "repos/{owner}/{repo}/issues/comments/{comment_id}",
            owner = self.owner,
            repo = self.repo,
            comment_id = comment_id
        );

        let response = self
            .crab
            ._delete(self.crab.absolute_url(route)?, None::<&()>)
            .await?;

        if response.status() == 204 {
            Ok(())
        } else {
            crate::map_github_error(response).await.map(drop)
        }
    }

    /// Lists comments in the issue.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .list_comments(101)
    ///     .since(chrono::Utc::now())
    ///     .per_page(100)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_comments(&self, issue_number: u64) -> ListCommentsBuilder<'_, '_> {
        ListCommentsBuilder::new(self, issue_number)
    }

    /// Lists comments for issues in the whole repo.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .issues("owner", "repo")
    ///     .list_issue_comments()
    ///     .per_page(100)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_issue_comments(&self) -> ListIssueCommentsBuilder<'_, '_> {
        ListIssueCommentsBuilder::new(self)
    }
}

#[derive(serde::Serialize)]
pub struct ListCommentsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    issue_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCommentsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, issue_number: u64) -> Self {
        Self {
            handler,
            issue_number,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Only comments updated at or after this time are returned.
    pub fn since(mut self, since: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::issues::Comment>> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue}/comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            issue = self.issue_number,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListIssueCommentsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListIssueCommentsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::issues::Comment>> {
        let route = format!(
            "repos/{owner}/{repo}/issues/comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}
