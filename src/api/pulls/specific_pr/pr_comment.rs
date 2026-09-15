use super::PullRequestHandler;
use crate::models::{pulls::ReviewComment, CommentId};
use serde_json::json;

pub struct SpecificPullRequestCommentBuilder<'octo, 'b> {
    handler: &'b PullRequestHandler<'octo>,
    comment_id: CommentId,
    pr_number: u64,
}

impl<'octo, 'b> SpecificPullRequestCommentBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b PullRequestHandler<'octo>,
        pr_number: u64,
        comment_id: CommentId,
    ) -> Self {
        Self {
            handler,
            comment_id,
            pr_number,
        }
    }

    /// Creates a reply to a review comment for the pull request.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#create-a-reply-for-a-review-comment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let reply = octocrab
    ///     .pulls("owner", "repo")
    ///     .pull_number(42)
    ///     .comment(123u64.into())
    ///     .reply("Looks good to me!")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reply(&self, comment: impl Into<String>) -> crate::Result<ReviewComment> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            comment_id = self.comment_id
        );
        self.handler
            .crab
            .post(route, Some(&json!({ "body": comment.into() })))
            .await
    }
}
