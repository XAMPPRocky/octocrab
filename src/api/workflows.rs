use crate::{models, Octocrab, Result};

pub struct WorkflowsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

/// Handler for GitHub's workflows API for actions.
///
/// Created with [`Octocrab::workflows`].
impl<'octo> WorkflowsHandler<'octo> {
    pub fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self {
            crab,
            owner: owner,
            repo: repo,
        }
    }
}
