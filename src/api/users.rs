//! The users API.

use std::backtrace::Backtrace;

use http::StatusCode;

pub use self::follow::{ListUserFollowerBuilder, ListUserFollowingBuilder};
pub use self::hovercard::HovercardBuilder;
pub use self::user_gpg_keys::{ListUserGpgKeysBuilder, UserGpgKeysOpsBuilder};
use self::user_repos::ListUserReposBuilder;
use crate::api::activity::starring::ListReposStarredByUserBuilder;
use crate::api::activity::watching::ListUserSubscriptionsBuilder;
use crate::api::events::EventsBuilder;
use crate::api::users::user_blocks::BlockedUsersBuilder;
use crate::api::users::user_emails::UserEmailsOpsBuilder;
use crate::api::users::user_git_ssh_keys::UserGitSshKeysOpsBuilder;
use crate::api::users::user_social_accounts::UserSocialAccountsOpsBuilder;
use crate::api::users::user_ssh_signing_keys::UserSshSigningKeysOpsBuilder;
use crate::models::UserId;
use crate::params::users::emails::EmailVisibilityState;
use crate::{error, GitHubError, Octocrab};

mod follow;
mod hovercard;
mod user_blocks;
mod user_emails;
mod user_git_ssh_keys;
mod user_gpg_keys;
mod user_repos;
mod user_social_accounts;
mod user_ssh_signing_keys;

pub(crate) enum UserRef {
    ByString(String),
    ById(UserId),
}

impl std::fmt::Display for UserRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRef::ByString(str) => write!(f, "users/{str}"),

            UserRef::ById(id) => write!(f, "user/{id}"),
        }
    }
}

/// Handler for GitHub's users API.
///
/// Created with [`Octocrab::users`].
pub struct UserHandler<'octo> {
    crab: &'octo Octocrab,
    user: UserRef,
}

impl<'octo> UserHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, user: UserRef) -> Self {
        Self { crab, user }
    }

    /// Handle packages for this user.
    ///
    /// See: https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28
    pub fn packages(&self) -> crate::api::packages::PackagesHandler<'octo> {
        let username = match &self.user {
            UserRef::ByString(name) => name.clone(),
            UserRef::ById(id) => id.to_string(),
        };
        crate::api::packages::PackagesHandler::new(
            self.crab,
            crate::api::packages::PackagesOwner::User(username),
        )
    }

    /// Get this users profile info
    pub async fn profile(&self) -> crate::Result<crate::models::UserProfile> {
        // build the route to get info on this user
        let route = format!("/{}", self.user);
        // get info on the specified user
        self.crab.get(route, None::<&()>).await
    }

    /// List this users that follow this user
    pub fn followers(&self) -> ListUserFollowerBuilder<'_, '_> {
        ListUserFollowerBuilder::new(self)
    }

    /// List this user is following
    pub fn following(&self) -> ListUserFollowingBuilder<'_, '_> {
        ListUserFollowingBuilder::new(self)
    }

    pub fn repos(&self) -> ListUserReposBuilder<'_, '_> {
        ListUserReposBuilder::new(self)
    }

    /// Lists organizations for the specified user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#list-organizations-for-a-user)
    pub fn list_orgs(&self) -> crate::api::current::ListUserOrgsBuilder<'octo> {
        crate::api::current::ListUserOrgsBuilder::new(self.crab, format!("/{}/orgs", self.user))
    }

    /// Lists repositories watched by this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/watching?apiVersion=2022-11-28#list-repositories-watched-by-a-user)
    pub fn subscriptions(&self) -> ListUserSubscriptionsBuilder<'octo> {
        ListUserSubscriptionsBuilder::new(self.crab, format!("/{}/subscriptions", self.user))
    }

    /// Lists repositories starred by this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28#list-repositories-starred-by-a-user)
    pub fn starred(&self) -> ListReposStarredByUserBuilder<'octo> {
        ListReposStarredByUserBuilder::with_route(self.crab, format!("/{}/starred", self.user))
    }

    /// List events for this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-events-for-the-authenticated-user)
    pub fn events(&self) -> EventsBuilder<'octo> {
        EventsBuilder::with_route(self.crab, format!("/{}/events", self.user))
    }

    /// List public events for this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-public-events-for-a-user)
    pub fn public_events(&self) -> EventsBuilder<'octo> {
        EventsBuilder::with_route(self.crab, format!("/{}/events/public", self.user))
    }

    /// List events received by this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-events-received-by-the-authenticated-user)
    pub fn received_events(&self) -> EventsBuilder<'octo> {
        EventsBuilder::with_route(self.crab, format!("/{}/received_events", self.user))
    }

    /// List public events received by this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-public-events-received-by-a-user)
    pub fn public_received_events(&self) -> EventsBuilder<'octo> {
        EventsBuilder::with_route(self.crab, format!("/{}/received_events/public", self.user))
    }

    /// List organization events for this user.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#list-organization-events-for-the-authenticated-user)
    pub fn org_events(&self, org: impl AsRef<str>) -> EventsBuilder<'octo> {
        EventsBuilder::with_route(
            self.crab,
            format!("/{}/events/orgs/{}", self.user, org.as_ref()),
        )
    }

    /// API for listing blocked users
    /// you must pass authentication information with your requests
    pub fn blocks(&self) -> BlockedUsersBuilder<'_, '_> {
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
        let route = format!("/user/blocks/{username}");
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
        let route = format!("/user/blocks/{username}");
        /* '204 not found' is returned if user blocked */
        let result: crate::Result<()> = self.crab.put(route, None::<&()>).await;
        match result {
            Ok(_) => Err(error::Error::GitHub {
                source: Box::new(GitHubError {
                    status_code: StatusCode::OK,
                    documentation_url: None,
                    errors: None,
                    message: "".to_string(),
                }),
                backtrace: Backtrace::capture(),
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
        let route = format!("/user/blocks/{username}");

        self.crab.delete(route, None::<&()>).await
    }

    ///## Set primary email visibility for the authenticated user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///
    ///"Email addresses" user permissions (write)
    ///
    ///```no_run
    ///  use octocrab::params::users::emails::EmailVisibilityState::*;
    ///  use octocrab::models::UserEmailInfo;
    ///
    ///  async fn run() -> octocrab::Result<Vec<UserEmailInfo>> {
    ///    octocrab::instance()
    ///    .users("current_user")
    ///    .set_primary_email_visibility(Public) // or Private
    ///    .await
    ///  }
    pub async fn set_primary_email_visibility(
        &self,
        visibility: EmailVisibilityState,
    ) -> crate::Result<Vec<crate::models::UserEmailInfo>> {
        let route = String::from("/user/email/visibility");
        let params = serde_json::json!({
            "visibility": serde_json::to_string(&visibility).unwrap(),
        });
        self.crab.patch(route, Some(&params)).await
    }

    ///Email addresses operations builder
    ///* List email addresses for the authenticated user
    ///* Add an email address for the authenticated user
    ///* Delete an email address for the authenticated user
    pub fn emails(&self) -> UserEmailsOpsBuilder<'_, '_> {
        UserEmailsOpsBuilder::new(self)
    }

    ///GPG Keys operations builder
    ///* List GPG keys for the authenticated user
    ///* Get a GPG key for the authenticated user
    ///* Add an GPG key for the authenticated user
    ///* Delete a GPG key for the authenticated user
    pub fn gpg_keys(&self) -> UserGpgKeysOpsBuilder<'_, '_> {
        UserGpgKeysOpsBuilder::new(self)
    }

    /// List GPG keys for the given user, allowing for pagination.
    ///
    /// See: [GitHub API Documentation][docs] for `GET /users/{username}/gpg_keys`
    ///
    /// # Examples
    ///
    /// * Fetch 10 recent GPG keys for the user with login "foouser":
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///     let gpg_keys = octocrab::instance()
    ///         .users("foouser")
    ///         .list_user_gpg_keys()
    ///         .page(1u32)
    ///         .per_page(10u8)
    ///         .send()
    ///         .await?;
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/users/gpg-keys?apiVersion=2022-11-28#list-gpg-keys-for-a-user
    pub fn list_user_gpg_keys(&self) -> ListUserGpgKeysBuilder<'_, '_> {
        ListUserGpgKeysBuilder::new(self)
    }

    ///Git SSH keys operations builder
    ///* List public SSH keys for the authenticated user
    ///* Create a public SSH key for the authenticated user
    ///* Delete a public SSH key for the authenticated user
    pub fn git_ssh_keys(&self) -> UserGitSshKeysOpsBuilder<'_, '_> {
        UserGitSshKeysOpsBuilder::new(self)
    }

    ///Social accounts operations builder
    ///* List social accounts for the authenticated user
    ///* Add social accounts for the authenticated user
    ///* Delete social accounts for the authenticated user
    pub fn social_accounts(&self) -> UserSocialAccountsOpsBuilder<'_, '_> {
        UserSocialAccountsOpsBuilder::new(self)
    }

    ///SSH signing key administration
    ///* List SSH signing keys for the authenticated user
    ///* Create an SSH signing key for the authenticated user
    ///* Get an SSH signing key for the authenticated user
    ///* Delete an SSH signing key for the authenticated user
    pub fn ssh_signing_keys(&self) -> UserSshSigningKeysOpsBuilder<'_, '_> {
        UserSshSigningKeysOpsBuilder::new(self)
    }

    /// Get contextual information for a user.
    ///
    /// Provides hovercard information. You can find out more about someone in
    /// relation to their pull requests, issues, repositories, and organizations.
    ///
    /// See: [GitHub API Documentation][docs] for `GET /users/{username}/hovercard`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///     let hovercard = octocrab::instance()
    ///         .users("octocat")
    ///         .hovercard()
    ///         .send()
    ///         .await?;
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-contextual-information-for-a-user
    pub fn hovercard(&self) -> HovercardBuilder<'_, '_> {
        HovercardBuilder::new(self)
    }
}
