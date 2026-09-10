use crate::models::Repository;
use crate::{Octocrab, Page};

/// A builder pattern struct for listing all public repositories.
///
/// Created by [`Octocrab::all_repositories`].
#[derive(serde::Serialize)]
pub struct ListAllRepositoriesBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<u64>,
}

impl<'octo> ListAllRepositoriesBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab, since: None }
    }

    /// A repository ID to start listing repositories from.
    pub fn since(mut self, since: impl Into<u64>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Repository>> {
        self.crab.get("/repositories", Some(&self)).await
    }
}
