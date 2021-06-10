/// A builder pattern struct for constructing an Octocrab request to create a
/// pull request.
#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct CreatePullRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b super::PullRequestHandler<'octo>,
    title: String,
    head: String,
    base: String,
    /// The contents of the pull request.
    body: Option<String>,
    /// Indicates whether the pull request is a draft.
    draft: Option<bool>,
    /// Indicates whether `maintainers` can modify the pull request.
    maintainer_can_modify: Option<bool>,
}

impl<'octo, 'b> CreatePullRequestBuilder<'octo, 'b> {
    /// Sends the request to create the pull request.
    pub async fn send(self) -> crate::Result<crate::models::pulls::PullRequest> {
        let url = format!(
            "repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );

        self.handler.http_post(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .create("test-pr", "master", "branch")
            .body(String::from("testing..."))
            .draft(true)
            .maintainer_can_modify(true);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "title": "test-pr",
                "head": "master",
                "base": "branch",
                "body": "testing...",
                "draft": true,
                "maintainer_can_modify": true,
            })
        )
    }
}
