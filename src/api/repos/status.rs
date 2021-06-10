use super::*;
use crate::models::StatusState;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct CreateStatusBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    pub sha: String,
    pub state: StatusState,
    /// A string label to differentiate this status from the status of other systems.
    pub context: Option<String>,
    /// The target URL to associate with this status. This URL will be linked from the GitHub UI to
    /// allow users to easily see the source of the status. For example, if your continuous
    /// integration system is posting build status, you would want to provide the deep link for the
    /// build output for this specific SHA: <http://ci.example.com/user/repo/build/sha>
    pub target_url: Option<String>,
    /// A short description of the status.
    pub description: Option<String>,
}

impl<'octo, 'r> CreateStatusBuilder<'octo, 'r> {
    /// The SHA hash of the target commit.
    pub fn sha(mut self, sha: String) -> Self {
        self.sha = sha;
        self
    }

    /// The state of the status.
    pub fn state(mut self, state: StatusState) -> Self {
        self.state = state;
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::FileUpdate> {
        let url = format!(
            "repos/{owner}/{repo}/statuses/{sha}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            sha = self.sha
        );
        self.handler.crab.put(url, Some(&self)).await
    }
}
