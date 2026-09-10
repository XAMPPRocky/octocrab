use super::RepoHandler;
use crate::models::{Repository, TeamId};

/// A builder pattern struct for transferring a repository to a new owner.
///
/// Created by [`RepoHandler::transfer`].
#[derive(serde::Serialize)]
pub struct TransferRepoBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    new_owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_ids: Option<Vec<TeamId>>,
}

impl<'octo, 'r> TransferRepoBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, new_owner: String) -> Self {
        Self {
            handler,
            new_owner,
            new_name: None,
            team_ids: None,
        }
    }

    /// The new name for the repository.
    pub fn new_name(mut self, new_name: impl Into<String>) -> Self {
        self.new_name = Some(new_name.into());
        self
    }

    /// ID of the teams to be given access to the repository.
    pub fn team_ids(mut self, team_ids: impl IntoIterator<Item = impl Into<TeamId>>) -> Self {
        self.team_ids = Some(team_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Repository> {
        let route = format!("/{}/transfer", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}
