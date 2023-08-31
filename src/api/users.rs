//! The users API.

mod user_repos;

pub use self::user_repos::ListUserReposBuilder;
use crate::Octocrab;

pub struct UserHandler<'octo> {
    crab: &'octo Octocrab,
    user: String,
}

impl<'octo> UserHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, user: String) -> Self {
        Self { crab, user }
    }

    pub fn repos(&self) -> ListUserReposBuilder<'_, '_> {
        ListUserReposBuilder::new(self)
    }
}
