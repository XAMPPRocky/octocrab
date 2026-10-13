//! Handler for GitHub's Stacked Pull Requests API.
//!
//! See <https://docs.github.com/en/rest/pulls/stacks?apiVersion=2022-11-28>

use http::StatusCode;

use crate::from_response::FromResponse;
use crate::models::pr_stacks::PrStack;

use super::RepoHandler;

/// A client to GitHub's Stacked Pull Requests API.
///
/// Created with [`RepoHandler::stacks`].
///
/// # Example
/// ```no_run
/// # async fn run() -> octocrab::Result<()> {
/// // List all stacks, filtering to those containing PR #42
/// let page = octocrab::instance()
///     .repos("owner", "repo")
///     .stacks()
///     .pull_request(42)
///     .per_page(10)
///     .list()
///     .await?;
/// # Ok(()) }
/// ```
pub struct StackedPrsHandler<'octo> {
    handler: &'octo RepoHandler<'octo>,
    /// Filter `list` to the stack containing this PR number.
    pull_request: Option<u64>,
    per_page: Option<u8>,
    page: Option<u32>,
}

// Internal serializable params struct for the list endpoint
#[derive(serde::Serialize)]
struct ListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> StackedPrsHandler<'octo> {
    pub(crate) fn new(handler: &'octo RepoHandler<'octo>) -> Self {
        Self {
            handler,
            pull_request: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter the list to stacks that contain this pull request number.
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let stacks = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .stacks()
    ///     .pull_request(42)
    ///     .list()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn pull_request(mut self, number: u64) -> Self {
        self.pull_request = Some(number);
        self
    }

    /// Results per page (max 100, default 30).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch (default 1).
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// List all pull request stacks in the repository.
    ///
    /// Returns a paginated list of [`PrStack`] objects. Use [`pull_request`],
    /// [`per_page`], and [`page`] to filter and paginate results.
    ///
    /// [`pull_request`]: StackedPrsHandler::pull_request
    /// [`per_page`]: StackedPrsHandler::per_page
    /// [`page`]: StackedPrsHandler::page
    ///
    /// # API
    /// `GET /repos/{owner}/{repo}/stacks`
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .stacks()
    ///     .per_page(50)
    ///     .list()
    ///     .await?;
    /// for stack in &page.items {
    ///     println!("Stack #{}: {} PRs", stack.number, stack.pull_requests.len());
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn list(&self) -> crate::Result<crate::Page<PrStack>> {
        let route = format!("/{}/stacks", self.handler.repo);
        let params = ListParams {
            pull_request: self.pull_request,
            per_page: self.per_page,
            page: self.page,
        };
        self.handler.crab.get(route, Some(&params)).await
    }

    /// Create a pull request stack from an ordered list of PR numbers.
    ///
    /// Provide pull request numbers from the **bottom** of the stack to the
    /// **top**. Each PR's base ref must match the previous PR's head ref.
    ///
    /// # API
    /// `POST /repos/{owner}/{repo}/stacks`
    ///
    /// # Errors
    /// Returns `422 Unprocessable Entity` if the PRs don't exist or can't
    /// form a valid stack.
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let stack = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .stacks()
    ///     .create(vec![101, 102, 103])
    ///     .await?;
    /// println!("Created stack #{}", stack.number);
    /// # Ok(()) }
    /// ```
    pub async fn create(&self, pull_requests: Vec<u64>) -> crate::Result<PrStack> {
        let route = format!("/{}/stacks", self.handler.repo);
        self.handler
            .crab
            .post(
                route,
                Some(&serde_json::json!({ "pull_requests": pull_requests })),
            )
            .await
    }

    /// Get a pull request stack by its stack number.
    ///
    /// # API
    /// `GET /repos/{owner}/{repo}/stacks/{stack_number}`
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let stack = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .stacks()
    ///     .get(1)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub async fn get(&self, stack_number: u64) -> crate::Result<PrStack> {
        let route = format!("/{}/stacks/{}", self.handler.repo, stack_number);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Append pull requests to the top of an existing stack.
    ///
    /// Provide only the PRs you want to add, ordered from the current top
    /// upward. The first new PR's base ref must match the current top PR's
    /// head ref.
    ///
    /// # API
    /// `POST /repos/{owner}/{repo}/stacks/{stack_number}/add`
    ///
    /// # Errors
    /// Returns `409 Conflict` if another modification is in progress, or
    /// `422 Unprocessable Entity` if the PRs can't be appended.
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let stack = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .stacks()
    ///     .add_to_stack(1, vec![104, 105])
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub async fn add_to_stack(
        &self,
        stack_number: u64,
        pull_requests: Vec<u64>,
    ) -> crate::Result<PrStack> {
        let route = format!("/{}/stacks/{}/add", self.handler.repo, stack_number);
        self.handler
            .crab
            .post(
                route,
                Some(&serde_json::json!({ "pull_requests": pull_requests })),
            )
            .await
    }

    /// Remove unmerged pull requests from a stack.
    ///
    /// Pull requests that cannot be unstacked (e.g. queued for merge) are left
    /// in place.
    ///
    /// - Returns `Some(stack)` (`200 OK`) when pull requests remain and the
    ///   updated stack is returned.
    /// - Returns `None` (`204 No Content`) when all pull requests have been
    ///   removed and the stack is dissolved.
    ///
    /// # API
    /// `POST /repos/{owner}/{repo}/stacks/{stack_number}/unstack`
    ///
    /// # Errors
    /// Returns `409 Conflict` if another modification is in progress, or
    /// `422 Unprocessable Entity` if every PR is locked.
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// match octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .stacks()
    ///     .remove_from_stack(1)
    ///     .await?
    /// {
    ///     Some(stack) => println!("Stack still has {} PRs", stack.pull_requests.len()),
    ///     None => println!("Stack dissolved"),
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn remove_from_stack(&self, stack_number: u64) -> crate::Result<Option<PrStack>> {
        let route = format!("/{}/stacks/{}/unstack", self.handler.repo, stack_number);
        let response = self.handler.crab._post(route, None::<&()>).await?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(None),
            _ => {
                let stack =
                    PrStack::from_response(crate::map_github_error(response).await?).await?;
                Ok(Some(stack))
            }
        }
    }
}
