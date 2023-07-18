//! The commit API.
mod create_comment;

pub use self::create_comment::CreateCommentBuilder;
use crate::{models, Octocrab, Result};

pub struct CommitHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> CommitHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    // pub fn create(&self, title: impl Into<String>) -> create::CreateIssueBuilder<'_, '_> {
    //     create::CreateIssueBuilder::new(self, title.into())
    // }

    pub fn create_comment(
        &self,
        sha: impl Into<String>,
        body: impl Into<String>,
    ) -> create_comment::CreateCommentBuilder<'_, '_> {
        create_comment::CreateCommentBuilder::new(self, sha.into(), body.into())
    }

    pub async fn get(&self, reference: impl Into<String>) -> Result<models::repos::RepoCommit> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.into(),
        );
        self.crab.get(route, None::<&()>).await
    }
}
