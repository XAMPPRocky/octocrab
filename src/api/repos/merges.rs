use super::*;

#[derive(serde::Serialize)]
pub struct MergeBranchBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    head: String,
    base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<String>,
}

impl<'octo, 'r> MergeBranchBuilder<'octo, 'r> {
    pub fn new(
        handler: &'r RepoHandler<'octo>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            head: head.into(),
            base: base.into(),
            commit_message: None,
        }
    }

    /// The message to use for the merge commit.
    pub fn commit_message(mut self, commit_message: impl Into<String>) -> Self {
        self.commit_message = Some(commit_message.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::MergeCommit> {
        let route = format!(
            "/repos/{owner}/{repo}/merges",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}
