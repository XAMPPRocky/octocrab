//! The gist comments API.
//!
//! Supports CRUD operations on comments for a gist.
//!
//! [Official documentation][docs]
//!
//! [docs]: https://docs.github.com/en/rest/gists/comments?apiVersion=2022-11-28

use http::StatusCode;

use crate::gists::GistsHandler;
use crate::models::gists::GistComment;
use crate::models::CommentId;
use crate::{Page, Result};

/// Handler for GitHub's gist comments API.
///
/// Created with [`GistsHandler::comments_for`].
#[derive(serde::Serialize)]
pub struct GistCommentsHandler<'octo, 'b> {
    #[serde(skip)]
    handler: &'b GistsHandler<'octo>,
    #[serde(skip)]
    gist_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> GistCommentsHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b GistsHandler<'octo>, gist_id: String) -> Self {
        Self {
            handler,
            gist_id,
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

    /// List the comments on a gist.
    ///
    /// See [GitHub API Documentation][docs] for more information.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comments = octocrab::instance()
    ///     .gists()
    ///     .comments_for("00_gist_id_00")
    ///     .per_page(30)
    ///     .page(1u32)
    ///     .list_comments()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/comments?apiVersion=2022-11-28#list-gist-comments
    pub async fn list_comments(&self) -> Result<Page<GistComment>> {
        let route = format!("/gists/{gist_id}/comments", gist_id = self.gist_id);
        self.handler.crab.get(route, Some(&self)).await
    }

    /// Create a comment on a gist.
    ///
    /// See [GitHub API Documentation][docs] for more information.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .gists()
    ///     .comments_for("00_gist_id_00")
    ///     .create_comment("This is a comment to a gist")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/comments?apiVersion=2022-11-28#create-a-gist-comment
    pub async fn create_comment(&self, comment_text: impl Into<String>) -> Result<GistComment> {
        let route = format!("/gists/{gist_id}/comments", gist_id = self.gist_id);
        let params = serde_json::json!({ "body": comment_text.into() });

        self.handler.crab.post(route, Some(&params)).await
    }

    /// Get a single comment on a gist.
    ///
    /// See [GitHub API Documentation][docs] for more information.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .gists()
    ///     .comments_for("00_gist_id_00")
    ///     .get_comment(1u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/comments?apiVersion=2022-11-28#get-a-gist-comment
    pub async fn get_comment(&self, comment_id: impl Into<CommentId>) -> Result<GistComment> {
        let comment_id = comment_id.into();
        let route = format!(
            "/gists/{gist_id}/comments/{comment_id}",
            gist_id = self.gist_id,
        );

        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a comment on a gist.
    ///
    /// See [GitHub API Documentation][docs] for more information.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let comment = octocrab::instance()
    ///     .gists()
    ///     .comments_for("00_gist_id_00")
    ///     .update_comment(1u64.into(), "This is an update to a comment in a gist")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/comments?apiVersion=2022-11-28#update-a-gist-comment
    pub async fn update_comment(
        &self,
        comment_id: impl Into<CommentId>,
        comment_text: impl Into<String>,
    ) -> Result<GistComment> {
        let comment_id = comment_id.into();
        let route = format!(
            "/gists/{gist_id}/comments/{comment_id}",
            gist_id = self.gist_id,
        );
        let params = serde_json::json!({ "body": comment_text.into() });

        self.handler.crab.patch(route, Some(&params)).await
    }

    /// Delete a comment on a gist.
    ///
    /// See [GitHub API Documentation][docs] for more information.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .gists()
    ///     .comments_for("00_gist_id_00")
    ///     .delete_comment(1u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/comments?apiVersion=2022-11-28#delete-a-gist-comment
    pub async fn delete_comment(&self, comment_id: impl Into<CommentId>) -> Result<()> {
        let comment_id = comment_id.into();
        let route = format!(
            "/gists/{gist_id}/comments/{comment_id}",
            gist_id = self.gist_id,
        );

        // DELETE here returns an empty body, ignore it since it doesn't make
        // sense to deserialize it as JSON.
        let response = self.handler.crab._delete(route, None::<&()>).await?;

        if response.status() != StatusCode::NOT_MODIFIED && !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }
}
