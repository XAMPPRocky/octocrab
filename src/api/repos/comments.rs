use super::RepoHandler;
use crate::models::{commits::Comment, CommentId};
use crate::{Page, Result};

/// A client to GitHub's repository commit comments API.
///
/// Created with [`RepoHandler::comments`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28)
pub struct RepoCommentsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoCommentsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists commit comments for a repository (or for a specific commit if configured).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#list-commit-comments-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let comments = octocrab.repos("owner", "repo")
    ///     .comments()
    ///     .list()
    ///     .per_page(10)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListRepoCommentsBuilder<'octo, 'r> {
        ListRepoCommentsBuilder::new(self.handler)
    }

    /// Gets a single commit comment by its ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#get-a-commit-comment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let comment = octocrab.repos("owner", "repo")
    ///     .comments()
    ///     .get(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, comment_id: impl Into<CommentId>) -> Result<Comment> {
        let comment_id = comment_id.into();
        let route = format!("/{}/comments/{comment_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Updates the body of a commit comment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#update-a-commit-comment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let comment = octocrab.repos("owner", "repo")
    ///     .comments()
    ///     .update(42, "Updated comment text")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        comment_id: impl Into<CommentId>,
        body: impl Into<String>,
    ) -> Result<Comment> {
        let comment_id = comment_id.into();
        let route = format!("/{}/comments/{comment_id}", self.handler.repo);
        let body = serde_json::json!({ "body": body.into() });
        self.handler.crab.patch(route, Some(&body)).await
    }

    /// Deletes a commit comment from a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#delete-a-commit-comment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.repos("owner", "repo")
    ///     .comments()
    ///     .delete(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, comment_id: impl Into<CommentId>) -> Result<()> {
        let comment_id = comment_id.into();
        let route = format!("/{}/comments/{comment_id}", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Creates a [`CreateRepoCommentBuilder`] to comment on a specific commit.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#create-a-commit-comment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let comment = octocrab.repos("owner", "repo")
    ///     .comments()
    ///     .create("commit_sha", "Great work!")
    ///     .path("src/lib.rs")
    ///     .position(10u64)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        commit_sha: impl Into<String>,
        body: impl Into<String>,
    ) -> CreateRepoCommentBuilder<'octo, 'r> {
        CreateRepoCommentBuilder::new(self.handler, commit_sha.into(), body.into())
    }
}

/// A builder pattern struct for listing commit comments for a repository or commit.
///
/// Created by [`RepoCommentsHandler::list`].
#[derive(serde::Serialize)]
pub struct ListRepoCommentsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRepoCommentsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            commit_sha: None,
            per_page: None,
            page: None,
        }
    }

    /// Scope the list of comments to a specific commit SHA.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#list-commit-comments)
    pub fn commit_sha(mut self, commit_sha: impl Into<String>) -> Self {
        self.commit_sha = Some(commit_sha.into());
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<Comment>> {
        let route = match &self.commit_sha {
            Some(sha) => format!("/{}/commits/{sha}/comments", self.handler.repo),
            None => format!("/{}/comments", self.handler.repo),
        };
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a commit comment for a repository.
///
/// Created by [`RepoCommentsHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateRepoCommentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    commit_sha: String,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u64>,
}

impl<'octo, 'r> CreateRepoCommentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, commit_sha: String, body: String) -> Self {
        Self {
            handler,
            commit_sha,
            body,
            path: None,
            position: None,
            line: None,
        }
    }

    /// Relative path of the file to comment on.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Line index in the diff to comment on.
    pub fn position(mut self, position: impl Into<u64>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Line number in the file to comment on.
    pub fn line(mut self, line: impl Into<u64>) -> Self {
        self.line = Some(line.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Comment> {
        let route = format!(
            "/{}/commits/{}/comments",
            self.handler.repo, self.commit_sha
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}
