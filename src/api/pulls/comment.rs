use serde_json::json;

use crate::models::{
    self,
    pulls::{Comment, Side},
    reactions::Reaction,
};

use super::*;

/// A builder pattern struct for listing comments.
///
/// created by [`PullRequestHandler::list_comments`]
///
/// [`PullRequestHandler::list_comments`]: ./struct.PullRequestHandler.html#method.list_comments
#[derive(serde::Serialize)]
pub struct ListCommentsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip)]
    pr: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<crate::params::pulls::comments::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
}

impl<'octo, 'b> ListCommentsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr: Option<u64>) -> Self {
        Self {
            handler,
            pr,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
            since: None,
        }
    }

    /// What to sort results by. Can be either `created` or `updated`,
    pub fn sort(mut self, sort: impl Into<crate::params::pulls::comments::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<crate::params::Direction>) -> Self {
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

    /// Only show notifications updated after the given time.
    pub fn since(mut self, since: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::pulls::Comment>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = if let Some(pr) = self.pr {
                pr.to_string() + "/"
            } else {
                "".into()
            },
        );
        self.handler.http_get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a comment.
///
/// created by [`PullRequestHandler::create_comment`]
///
/// [`PullRequestHandler::create_comment`]: ./struct.PullRequestHandler.html#method.create_comment
#[derive(serde::Serialize)]
pub struct CreateCommentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::PullRequestHandler<'octo>,
    #[serde(skip)]
    pr: u64,
    commit_id: String,
    body: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<CommentId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_type: Option<String>,
}

impl<'octo, 'r> CreateCommentBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r super::PullRequestHandler<'octo>,
        pr: u64,
        commit_id: String,
        body: String,
        path: String,
    ) -> Self {
        Self {
            handler,
            pr,
            commit_id,
            body,
            path,
            position: None,
            line: None,
            start_line: None,
            start_side: None,
            side: None,
            in_reply_to: None,
            subject_type: None,
        }
    }

    /// Sends the actual request.
    /// https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#create-a-review-comment-for-a-pull-request
    pub async fn send(self) -> crate::Result<models::pulls::Comment> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr,
        );

        self.handler.crab.post(route, Some(&self)).await
    }

    pub fn position(mut self, position: impl Into<Option<u64>>) -> Self {
        self.position = position.into();
        self
    }

    pub fn line(mut self, line: impl Into<Option<u64>>) -> Self {
        self.line = line.into();
        self
    }

    pub fn start_line(mut self, start_line: impl Into<Option<u64>>) -> Self {
        self.start_line = start_line.into();
        self
    }

    pub fn start_side(mut self, start_side: impl Into<Option<Side>>) -> Self {
        self.start_side = start_side.into();
        self
    }

    pub fn side(mut self, side: impl Into<Option<Side>>) -> Self {
        self.side = side.into();
        self
    }

    pub fn in_reply_to(mut self, in_reply_to: impl Into<Option<CommentId>>) -> Self {
        self.in_reply_to = in_reply_to.into();
        self
    }

    pub fn subject_type(mut self, subject_type: impl Into<Option<String>>) -> Self {
        self.subject_type = subject_type.into();
        self
    }
}

/// A builder pattern struct for working with specific comment.
///
/// created by [`PullRequestHandler::comment`]
///
/// [`PullRequestHandler::comment`]: ./struct.PullRequestHandler.html#method.comment
#[derive(serde::Serialize)]
pub struct CommentBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    comment_id: CommentId,
}

impl<'octo, 'b> CommentBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, comment_id: CommentId) -> Self {
        Self {
            handler,
            comment_id,
        }
    }

    ///https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#get-a-review-comment-for-a-pull-request
    pub async fn get(self) -> crate::Result<Comment> {
        self.handler
            .crab
            .get(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                None::<&Comment>,
            )
            .await
    }

    ///https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#update-a-review-comment-for-a-pull-request
    pub async fn update(self, comment: &str) -> crate::Result<Comment> {
        self.handler
            .crab
            .patch(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                Some(&json!({ "body": comment })),
            )
            .await
    }

    ///https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28#create-reaction-for-a-pull-request-review-comment
    pub async fn react(
        self,
        reaction: models::reactions::ReactionContent,
    ) -> crate::Result<Reaction> {
        self.handler
            .crab
            .post(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                Some(&json!({ "content": reaction })),
            )
            .await
    }

    ///https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28#delete-a-pull-request-comment-reaction
    pub async fn delete_react(self, reaction_id: u64) -> crate::Result<()> {
        self.handler
            .crab
            ._delete(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id,
                    reaction_id = reaction_id
                ),
                None::<&()>,
            )
            .await?;
        Ok(())
    }

    ///https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#delete-a-review-comment-for-a-pull-request
    pub async fn delete(self) -> crate::Result<()> {
        self.handler
            .crab
            ._delete(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                None::<&()>,
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let yesterday = chrono::Utc::now() - chrono::Duration::days(1);
        let list = handler
            .list_comments(Some(1))
            .sort(crate::params::pulls::comments::Sort::Updated)
            .direction(crate::params::Direction::Ascending)
            .since(yesterday)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "sort": "updated",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
                "since": yesterday
            })
        )
    }
}
