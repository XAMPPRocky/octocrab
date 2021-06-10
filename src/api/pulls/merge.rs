use super::*;

/// A builder pattern struct for merging pull requests.
///
/// created by [`PullRequestHandler::merge`]
///
/// [`PullRequestHandler::merge`]: ./struct.PullRequestHandler.html#method.merge
#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct MergePullRequestsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip)]
    pr_number: u64,
    /// Title for the automatic commit message.
    #[builder(rename = "title")]
    commit_title: Option<String>,
    /// Extra detail to append to automatic commit message.
    #[builder(rename = "message")]
    commit_message: Option<String>,
    /// SHA that pull request head must match to allow merge.
    sha: Option<String>,
    /// Merge method to use. Default is `Merge`.
    #[builder(rename = "method")]
    merge_method: Option<crate::params::pulls::MergeMethod>,
}

impl<'octo, 'b> MergePullRequestsBuilder<'octo, 'b> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::pulls::Merge> {
        let url = format!(
            "repos/{owner}/{repo}/pulls/{pull_number}/merge",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
        );

        self.handler.http_put(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let merge = handler
            .merge(80818)
            .title("just testing!")
            .message("promise!")
            .sha("luckily this won't deserialize ;)")
            .method(crate::params::pulls::MergeMethod::Squash);

        assert_eq!(
            serde_json::to_value(merge).unwrap(),
            serde_json::json!({
                "commit_title": "just testing!",
                "commit_message": "promise!",
                "sha": "luckily this won't deserialize ;)",
                "merge_method": "squash",
            })
        )
    }
}
