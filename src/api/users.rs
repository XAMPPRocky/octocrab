//! The users API.

use std::backtrace::Backtrace;

use http::StatusCode;
use snafu::GenerateImplicitData;

use crate::api::users::user_blocks::BlockedUsersBuilder;
use crate::{error, GitHubError, Octocrab};

pub use self::follow::{ListUserFollowerBuilder, ListUserFollowingBuilder};
use self::user_repos::ListUserReposBuilder;

mod follow;
mod user_blocks;
mod user_repos;

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

    /// API for listing blocked users
    /// you must pass authentication information with your requests
    pub fn blocks(&self) -> BlockedUsersBuilder {
        BlockedUsersBuilder::new(self)
    }

    ///## Check if a user is blocked by the authenticated user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The token must have the following permission set: `blocking:read`
    ///
    ///```no_run
    ///  async fn run() -> octocrab::Result<bool> {
    ///    let is_blocked = octocrab::instance()
    ///        .users("current_user")
    ///        .is_blocked("some_user")
    ///        .await?;
    ///    Ok(is_blocked)
    ///  }
    pub async fn is_blocked(&self, username: &str) -> crate::Result<bool> {
        let route = format!("/user/blocks/{}", username);
        let response = self.crab._get(route).await?;
        Ok(response.status() == 204)
    }

    ///## Blocks the given user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The token must have the following permission set: `blocking:read`
    ///
    ///```no_run
    ///  async fn run() -> octocrab::Result<()> {
    ///    octocrab::instance()
    ///    .users("current_user")
    ///    .block_user("some_user")
    ///    .await
    ///  }
    pub async fn block_user(&self, username: &str) -> crate::Result<()> {
        let route = format!("/user/blocks/{}", username);
        /* '204 not found' is returned if user blocked */
        let result: crate::Result<()> = self.crab.put(route, None::<&()>).await;
        match result {
            Ok(_) => Err(error::Error::GitHub {
                source: GitHubError {
                    status_code: StatusCode::OK,
                    documentation_url: None,
                    errors: None,
                    message: "".to_string(),
                },
                backtrace: Backtrace::generate(),
            }),
            Err(_v) => Ok(()),
        }
    }

    ///## Unblocks the given user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The token must have the following permission set: `blocking:read`
    ///
    ///```no_run
    ///  async fn run() -> octocrab::Result<()> {
    ///    octocrab::instance()
    ///    .users("current_user")
    ///    .unblock_user("some_user")
    ///    .await
    ///  }
    pub async fn unblock_user(&self, username: &str) -> crate::Result<()> {
        let route = format!("/user/blocks/{}", username);

        self.crab.delete(route, None::<&()>).await
    }
}
