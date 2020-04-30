use crate::{models, Octocrab};

pub struct IssueHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> IssueHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// Adds up to 10 assignees to an issue. Users already assigned to an issue
    /// are not replaced.
    pub async fn add_assignees(&self, number: u64, assignees: &[u64]) -> crate::Result<models::Issue> {
        let route = format!(
            "/repos/{owner}/{repo}/issues/{issue}/assignees",
            owner = self.owner,
            repo = self.repo,
            issue = number
        );

        self.crab
            .post(
                self.crab.absolute_url(route)?,
                Some(&serde_json::json!({ "assignees": assignees })),
            )
            .await
    }
}
