//! The users API.

mod follow;
mod user_repos;

use self::follow::{ListUserFollowerBuilder, ListUserFollowingBuilder};
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

    /// Get this users profile info
    pub async fn profile(&self) -> crate::Result<crate::models::UserProfile> {
        // build the route to get info on this user
        let route = format!("/users/{}", self.user);
        // get info on the specified user
        self.crab.get(route, None::<&()>).await
    }

    /// List this users that follow this user
    pub fn followers(&self) -> ListUserFollowerBuilder {
        ListUserFollowerBuilder::new(self)
    }

    /// List this user is following
    pub fn following(&self) -> ListUserFollowingBuilder {
        ListUserFollowingBuilder::new(self)
    }

    pub fn repos(&self) -> ListUserReposBuilder<'_, '_> {
        ListUserReposBuilder::new(self)
    }
}
