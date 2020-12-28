use super::*;
use crate::models::{Status, StatusState, User};
use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct CreateStatusBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    pub sha: String,
    pub state: StatusState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl<'octo, 'r> CreateStatusBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, sha: String, state: StatusState) -> Self {
        Self {
            handler,
            sha,
            state,
            context: None,
            target_url: None,
            description: None,
        }
    }

    pub fn sha(mut self, sha: String) -> Self {
        self.sha = sha;
        self
    }

    pub fn context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn target(mut self, target: String) -> Self {
        self.target_url = Some(target);
        self
    }

    pub fn state(mut self, state: StatusState) -> Self {
        self.state = state;
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::FileUpdate> {
        let url = format!(
            "/repos/{owner}/{repo}/statuses/{sha}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            sha = self.sha
        );
        self.handler.crab.put(url, Some(&self)).await
    }
}
