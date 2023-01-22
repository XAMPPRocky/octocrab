//! The commit API.
mod create_comment;

use crate::{models, Octocrab};
pub use self::{
    create_comment::CreateCommentBuilder,
};

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
}
