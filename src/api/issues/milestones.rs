use super::IssueHandler;
use crate::models::{Label, Milestone};
use crate::{params, Result};
use chrono::{DateTime, Utc};

/// Handler for GitHub's repository milestones API.
///
/// Created with [`IssueHandler::milestones`].
pub struct MilestonesHandler<'octo, 'r> {
    handler: &'r IssueHandler<'octo>,
}

impl<'octo, 'r> MilestonesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List milestones for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28#list-milestones)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let milestones = octocrab.issues("owner", "repo")
    ///     .milestones()
    ///     .list()
    ///     .state(params::State::Open)
    ///     .sort(params::milestones::Sort::DueOn)
    ///     .direction(params::Direction::Ascending)
    ///     .per_page(100)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListMilestonesBuilder<'octo, 'r> {
        ListMilestonesBuilder::new(self.handler)
    }

    /// Create a milestone in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28#create-a-milestone)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let milestone = octocrab.issues("owner", "repo")
    ///     .milestones()
    ///     .create("v1.0")
    ///     .description("Release version 1.0")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, title: impl Into<String>) -> CreateMilestoneBuilder<'octo, 'r> {
        CreateMilestoneBuilder::new(self.handler, title.into())
    }

    /// Get a milestone from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28#get-a-milestone)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let milestone = octocrab.issues("owner", "repo")
    ///     .milestones()
    ///     .get(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, milestone_number: i64) -> Result<Milestone> {
        let route = format!(
            "/{repo}/milestones/{milestone_number}",
            repo = self.handler.repo,
            milestone_number = milestone_number,
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a milestone in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28#update-a-milestone)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let milestone = octocrab.issues("owner", "repo")
    ///     .milestones()
    ///     .update(1)
    ///     .title("v1.0.1")
    ///     .state(params::State::Closed)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, milestone_number: i64) -> UpdateMilestoneBuilder<'octo, 'r> {
        UpdateMilestoneBuilder::new(self.handler, milestone_number)
    }

    /// Delete a milestone from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28#delete-a-milestone)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.issues("owner", "repo")
    ///     .milestones()
    ///     .delete(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, milestone_number: i64) -> Result<()> {
        let route = format!(
            "/{repo}/milestones/{milestone_number}",
            repo = self.handler.repo,
            milestone_number = milestone_number,
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List labels for issues in a milestone.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28#list-labels-for-issues-in-a-milestone)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let labels = octocrab.issues("owner", "repo")
    ///     .milestones()
    ///     .list_labels(1)
    ///     .per_page(100)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_labels(&self, milestone_number: i64) -> ListMilestoneLabelsBuilder<'octo, 'r> {
        ListMilestoneLabelsBuilder::new(self.handler, milestone_number)
    }
}

#[derive(serde::Serialize)]
pub struct ListMilestonesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<params::milestones::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListMilestonesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter milestones by `state`.
    pub fn state(mut self, state: params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// What to sort results by.
    pub fn sort(mut self, sort: impl Into<params::milestones::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort.
    pub fn direction(mut self, direction: impl Into<params::Direction>) -> Self {
        self.direction = Some(direction.into());
        self
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
    pub async fn send(self) -> Result<crate::Page<Milestone>> {
        let route = format!("/{repo}/milestones", repo = self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct CreateMilestoneBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_on: Option<DateTime<Utc>>,
}

impl<'octo, 'r> CreateMilestoneBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, title: String) -> Self {
        Self {
            handler,
            title,
            state: None,
            description: None,
            due_on: None,
        }
    }

    /// The state of the milestone. Either `open` or `closed`.
    pub fn state(mut self, state: params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// A description of the milestone.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The milestone due date.
    pub fn due_on(mut self, due_on: impl Into<DateTime<Utc>>) -> Self {
        self.due_on = Some(due_on.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<Milestone> {
        let route = format!("/{repo}/milestones", repo = self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct UpdateMilestoneBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    milestone_number: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_on: Option<DateTime<Utc>>,
}

impl<'octo, 'r> UpdateMilestoneBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, milestone_number: i64) -> Self {
        Self {
            handler,
            milestone_number,
            title: None,
            state: None,
            description: None,
            due_on: None,
        }
    }

    /// The title of the milestone.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// The state of the milestone. Either `open` or `closed`.
    pub fn state(mut self, state: params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// A description of the milestone.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The milestone due date.
    pub fn due_on(mut self, due_on: impl Into<DateTime<Utc>>) -> Self {
        self.due_on = Some(due_on.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<Milestone> {
        let route = format!(
            "/{repo}/milestones/{milestone_number}",
            repo = self.handler.repo,
            milestone_number = self.milestone_number,
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListMilestoneLabelsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    milestone_number: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListMilestoneLabelsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, milestone_number: i64) -> Self {
        Self {
            handler,
            milestone_number,
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
    pub async fn send(self) -> Result<crate::Page<Label>> {
        let route = format!(
            "/{repo}/milestones/{milestone_number}/labels",
            repo = self.handler.repo,
            milestone_number = self.milestone_number,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
