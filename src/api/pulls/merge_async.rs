//! Asynchronous pull request merge builder.
//!
//! See <https://docs.github.com/en/rest/pulls/pulls#merge-a-pull-request-asynchronously>

use crate::models::pr_stacks::{AsyncMergeAction, AsyncMergeMethod, AsyncMergeResult};

use super::PullRequestHandler;

/// Builder for submitting an asynchronous pull request merge.
///
/// Created with [`PullRequestHandler::merge_async`].
///
/// Merging asynchronously allows complex merges to be retried and avoids
/// request timeouts. This is the **required** method for merging stacked pull
/// requests — when merging a stacked PR, all PRs below it in the stack are
/// also merged.
///
/// # GitHub Documentation
/// <https://docs.github.com/en/rest/pulls/pulls#merge-a-pull-request-asynchronously>
///
/// # Example
/// ```no_run
/// # async fn run() -> octocrab::Result<()> {
/// use octocrab::models::pr_stacks::{AsyncMergeMethod, AsyncMergeStatus};
///
/// let result = octocrab::instance()
///     .pulls("owner", "repo")
///     .merge_async(42)
///     .merge_method(AsyncMergeMethod::Squash)
///     .send()
///     .await?;
///
/// println!("Merge status: {:?}", result.status);
/// # Ok(()) }
/// ```
#[derive(serde::Serialize)]
pub struct MergeAsyncBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip)]
    pr_number: u64,
    /// Title for the automatic commit message.
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_title: Option<String>,
    /// Extra detail to append to the automatic commit message.
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<String>,
    /// SHA that the PR head must match at merge time. If omitted, the current
    /// head SHA is used; if the PR is pushed to between submission and
    /// execution, the merge is cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    /// Merge method to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<AsyncMergeMethod>,
    /// Merge action strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_action: Option<AsyncMergeAction>,
}

impl<'octo, 'b> MergeAsyncBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            pr_number,
            commit_title: None,
            commit_message: None,
            sha: None,
            merge_method: None,
            merge_action: None,
        }
    }

    /// Title for the automatic commit message.
    pub fn commit_title(mut self, title: impl Into<String>) -> Self {
        self.commit_title = Some(title.into());
        self
    }

    /// Extra detail to append to the automatic commit message.
    pub fn commit_message(mut self, msg: impl Into<String>) -> Self {
        self.commit_message = Some(msg.into());
        self
    }

    /// SHA that the pull request head must match to allow merge.
    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    /// Merge method to use (merge, squash, or rebase).
    pub fn merge_method(mut self, method: AsyncMergeMethod) -> Self {
        self.merge_method = Some(method);
        self
    }

    /// Merge action strategy (default, direct_merge, or merge_queue).
    pub fn merge_action(mut self, action: AsyncMergeAction) -> Self {
        self.merge_action = Some(action);
        self
    }

    /// Submit the asynchronous merge request.
    ///
    /// Returns the current merge result. A `200` response means the PR was
    /// already merged or is already in a merge queue; `202` means the request
    /// was accepted and is running in the background. Both cases return an
    /// [`AsyncMergeResult`].
    pub async fn send(self) -> crate::Result<AsyncMergeResult> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/merge-async",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = self.pr_number,
        );
        self.handler.http_put(route, Some(&self)).await
    }
}
