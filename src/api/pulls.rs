//! The pull request API.

mod create;
mod list;

use crate::{Octocrab, Page};

pub use self::{create::CreatePullRequestBuilder, list::ListPullRequestsBuilder};

/// A client to GitHub's pull request API.
///
/// Created with [`Octocrab::pulls`].
///
/// [`Octocrab::pulls`]: ../struct.Octocrab.html#method.pulls
pub struct PullRequestHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
    media_type: Option<crate::params::pulls::MediaType>,
}

impl<'octo> PullRequestHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo, media_type: None }
    }

    /// Set the media type for this request.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let pr = octocrab::instance()
    ///     .pulls("owner", "repo")
    ///     .media_type(octocrab::params::pulls::MediaType::Diff)
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
        let response = self
            .crab
            ._get(self.crab.absolute_url(route)?, None::<&()>)
            .await?;

        Ok(response.status() == 204)
    }

    /// Get's a given pull request with by its `pr` number.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let pr = octocrab.pulls("owner", "repo").get(101).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, pr: u64) -> crate::Result<crate::models::PullRequest> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );

        self.http_get(url, None::<&()>).await
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
}

impl<'octo> PullRequestHandler<'octo> {
    pub(crate) async fn http_get<R, A, P>(&self, route: A, parameters: Option<&P>) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let mut request = self.crab.client.get(self.crab.absolute_url(route)?);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        if let Some(media_type) = self.media_type {
            request = request.header(reqwest::header::ACCEPT, crate::format_media_type(&media_type.to_string()));
        }

        R::from_response(crate::Octocrab::map_github_error(self.crab.execute(request).await?).await?).await
    }

    pub(crate) async fn http_post<R, A, P>(&self, route: A, body: Option<&P>) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let mut request = self.crab.client.post(self.crab.absolute_url(route)?);

        if let Some(body) = body {
            request = request.json(body);
        }

        if let Some(media_type) = self.media_type {
            request = request.header(reqwest::header::ACCEPT, crate::format_media_type(&media_type.to_string()));
        }

        R::from_response(crate::Octocrab::map_github_error(self.crab.execute(request).await?).await?).await
    }

}
