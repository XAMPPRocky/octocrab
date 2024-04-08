//! The pull request API.

mod comment;
mod create;
mod list;
mod merge;
mod update;

use http::request::Builder;
use http::{Method, Uri};

use snafu::ResultExt;

use crate::error::HttpSnafu;
use crate::{Octocrab, Page};

pub use self::{
    create::CreatePullRequestBuilder, list::ListPullRequestsBuilder,
    update::UpdatePullRequestBuilder,
};

/// A client to GitHub's pull request API.
///
/// Created with [`Octocrab::pulls`].
pub struct PullRequestHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
    media_type: Option<crate::params::pulls::MediaType>,
}

impl<'octo> PullRequestHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self {
            crab,
            owner,
            repo,
            media_type: None,
        }
    }

    /// Set the media type for this request.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let pr = octocrab::instance()
    ///     .pulls("owner", "repo")
    ///     .media_type(octocrab::params::pulls::MediaType::Full)
    ///     .get(404)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn media_type(mut self, media_type: crate::params::pulls::MediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    /// Checks if a given pull request has been merged.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.pulls("owner", "repo").is_merged(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_merged(&self, pr: u64) -> crate::Result<bool> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/merge",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(crate::error::HttpSnafu)?;
        let response = self.crab._get(uri).await?;

        Ok(response.status() == 204)
    }

    /// Update the branch of a pull request.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.pulls("owner", "repo").update_branch(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_branch(&self, pr: u64) -> crate::Result<bool> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/update-branch",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(crate::error::HttpSnafu)?;
        let response = self.crab._put(uri, None::<&()>).await?;

        Ok(response.status() == 202)
    }

    /// Get's a given pull request with by its `pr` number.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let pr = octocrab::instance().pulls("owner", "repo").get(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, pr: u64) -> crate::Result<crate::models::pulls::PullRequest> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );

        self.http_get(route, None::<&()>).await
    }

    /// Get's a given pull request's `diff`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let diff = octocrab::instance().pulls("owner", "repo").get_diff(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_diff(&self, pr: u64) -> crate::Result<String> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(crate::error::HttpSnafu)?;
        let request = Builder::new()
            .method(Method::GET)
            .uri(uri)
            .header(http::header::ACCEPT, crate::format_media_type("diff"));
        let request = self.crab.build_request(request, None::<&()>)?;
        let response = crate::map_github_error(self.crab.execute(request).await?).await?;
        self.crab.body_to_string(response).await
    }

    /// Get's a given pull request's patch.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let diff = octocrab::instance().pulls("owner", "repo").get_patch(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_patch(&self, pr: u64) -> crate::Result<String> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(crate::error::HttpSnafu)?;
        let request = Builder::new()
            .method("GET")
            .uri(uri)
            .header(http::header::ACCEPT, crate::format_media_type("patch"));
        let request = self.crab.build_request(request, None::<&()>)?;
        let response = crate::map_github_error(self.crab.execute(request).await?).await?;

        self.crab.body_to_string(response).await
    }

    /// Create a new pull request.
    ///
    /// - `title` — The title of the new pull request.
    /// - `head` — The name of the branch where your changes are implemented.
    ///   For cross-repository pull requests in the same network, namespace head
    ///   with a user like this: `username:branch`.
    /// - `base` — The name of the branch you want the changes pulled into. This
    ///   should be an existing branch on the current repository. You cannot
    ///   submit a pull request to one repository that requests a merge to a
    ///   base of another repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab
    ///     .pulls("owner", "repo")
    ///     .create("title", "head", "base")
    ///     .body("hello world!")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        title: impl Into<String>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> create::CreatePullRequestBuilder<'octo, '_> {
        create::CreatePullRequestBuilder::new(self, title, head, base)
    }

    /// Update a new pull request.
    ///
    /// - `pull_number` — pull request number.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab
    ///     .pulls("owner", "repo")
    ///     .update(1)
    ///     .body("hello world!")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, pull_number: u64) -> update::UpdatePullRequestBuilder<'octo, '_> {
        update::UpdatePullRequestBuilder::new(self, pull_number)
    }

    /// Creates a new `ListPullRequestsBuilder` that can be configured to filter
    /// listing pulling requests.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let page = octocrab.pulls("owner", "repo").list()
    ///     // Optional Parameters
    ///     .state(params::State::Open)
    ///     .head("master")
    ///     .base("branch")
    ///     .sort(params::pulls::Sort::Popularity)
    ///     .direction(params::Direction::Ascending)
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> list::ListPullRequestsBuilder {
        list::ListPullRequestsBuilder::new(self)
    }

    /// Lists all of the `Review`s associated with the pull request.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let reviews = octocrab::instance()
    ///     .pulls("owner", "repo")
    ///     .list_reviews(21u64.into())
    ///     .per_page(100)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_reviews(&self, pr_number: u64) -> ListReviewsBuilder<'_, '_> {
        ListReviewsBuilder::new(self, pr_number)
    }

    /// Request a review from users or teams.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let review = octocrab::instance().pulls("owner", "repo")
    ///    .request_reviews(101, ["user1".to_string(), "user2".to_string()], ["team1".to_string(), "team2".to_string()])
    ///  .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request_reviews(
        &self,
        pr: u64,
        reviewers: impl Into<Vec<String>>,
        team_reviewers: impl Into<Vec<String>>,
    ) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/requested_reviewers",
            owner = self.owner,
            repo = self.repo,
        );

        let mut map = serde_json::Map::new();
        map.insert("reviewers".to_string(), reviewers.into().into());
        map.insert("team_reviewers".to_string(), team_reviewers.into().into());

        self.crab.post(route, Some(&map)).await
    }

    /// Remove a requested reviewer from users or teams.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let review = octocrab::instance().pulls("owner", "repo")
    ///    .remove_requested_reviewers(101, ["user1".to_string(), "user2".to_string()], ["team1".to_string(), "team2".to_string()])
    ///  .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_requested_reviewers(
        &self,
        pr: u64,
        reviewers: impl Into<Vec<String>>,
        team_reviewers: impl Into<Vec<String>>,
    ) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/requested_reviewers",
            owner = self.owner,
            repo = self.repo,
        );

        let mut map = serde_json::Map::new();
        map.insert("reviewers".to_string(), reviewers.into().into());
        map.insert("team_reviewers".to_string(), team_reviewers.into().into());

        self.crab.delete(route, Some(&map)).await
    }

    /// List all `DiffEntry`s associated with the pull request.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let files = octocrab::instance().pulls("owner", "repo").list_files(101).await?;
    /// # Ok(())
    /// # }
    pub async fn list_files(
        &self,
        pr: u64,
    ) -> crate::Result<Page<crate::models::repos::DiffEntry>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/files",
            owner = self.owner,
            repo = self.repo,
        );

        self.http_get(route, None::<&()>).await
    }

    /// Creates a new `ListCommentsBuilder` that can be configured to list and
    /// filter `Comments` for a particular pull request. If no pull request is
    /// specified, lists comments for the whole repo.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let page = octocrab.pulls("owner", "repo").list_comments(Some(5))
    ///     // Optional Parameters
    ///     .sort(params::pulls::comments::Sort::Created)
    ///     .direction(params::Direction::Ascending)
    ///     .per_page(100)
    ///     .page(5u32)
    ///     .since(chrono::Utc::now() - chrono::Duration::days(1))
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_comments(&self, pr: Option<u64>) -> comment::ListCommentsBuilder {
        comment::ListCommentsBuilder::new(self, pr)
    }

    /// Creates a new `MergePullRequestsBuilder` that can be configured used to
    /// merge a pull request.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let page = octocrab.pulls("owner", "repo").merge(20)
    ///     // Optional Parameters
    ///     .title("cool title")
    ///     .message("a message")
    ///     // Won't merge of the HEAD commit of the PR branch is not the same
    ///     .sha("0123456")
    ///     // The method to use when merging, will default to `Merge`
    ///     .method(params::pulls::MergeMethod::Squash)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn merge(&self, pr: u64) -> merge::MergePullRequestsBuilder {
        merge::MergePullRequestsBuilder::new(self, pr)
    }
}

impl<'octo, 'r> ListReviewsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            pr_number,
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::pulls::Review>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/reviews",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = self.pr_number,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListReviewsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r PullRequestHandler<'octo>,
    #[serde(skip)]
    pr_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> PullRequestHandler<'octo> {
    pub(crate) async fn http_get<R, A, P>(
        &self,
        route: A,
        parameters: Option<&P>,
    ) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let uri = self.crab.parameterized_uri(route, parameters)?;

        let mut request = Builder::new().uri(uri);
        if let Some(media_type) = self.media_type {
            request = request.header(
                http::header::ACCEPT,
                crate::format_media_type(media_type.to_string()),
            );
        }
        let request = self.crab.build_request(request, None::<&()>)?;

        R::from_response(crate::map_github_error(self.crab.execute(request).await?).await?).await
    }

    pub(crate) async fn http_post<R, A, P>(&self, route: A, body: Option<&P>) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let uri = Uri::builder()
            .path_and_query(route.as_ref())
            .build()
            .context(HttpSnafu)?;
        let mut request = Builder::new().method(Method::POST).uri(uri);
        request = self.build_request(request);
        let request = self.crab.build_request(request, body)?;

        R::from_response(crate::map_github_error(self.crab.execute(request).await?).await?).await
    }

    pub(crate) async fn http_put<R, A, P>(&self, route: A, body: Option<&P>) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let uri = Uri::builder()
            .path_and_query(route.as_ref())
            .build()
            .context(HttpSnafu)?;
        let mut request = Builder::new().method(Method::PUT).uri(uri);

        request = self.build_request(request);
        let request = self.crab.build_request(request, body)?;

        R::from_response(crate::map_github_error(self.crab.execute(request).await?).await?).await
    }

    pub(crate) async fn http_patch<R, A, P>(&self, route: A, body: Option<&P>) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let uri = Uri::builder()
            .path_and_query(route.as_ref())
            .build()
            .context(HttpSnafu)?;
        let mut request = Builder::new().method(Method::PATCH).uri(uri);

        request = self.build_request(request);
        let request = self.crab.build_request(request, body)?;
        R::from_response(crate::map_github_error(self.crab.execute(request).await?).await?).await
    }

    fn build_request(&self, mut request: http::request::Builder) -> http::request::Builder
where {
        if let Some(media_type) = self.media_type {
            request = request.header(
                http::header::ACCEPT,
                crate::format_media_type(media_type.to_string()),
            );
        }
        request
    }
}
