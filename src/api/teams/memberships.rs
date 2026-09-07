use http::Uri;
use snafu::ResultExt;

use crate::error::HttpSnafu;
use crate::models::memberships::{Role, TeamMembership};
use crate::{Octocrab, Result};

#[derive(Debug, serde::Serialize)]
struct RoleUpdateBody {
    role: Role,
}

/// Handler for managing a team's memberships through
/// GitHub's teams API.
///
/// Created with [`TeamHandler::memberships`].
///
/// [`TeamHandler::memberships`]: crate::api::teams::TeamHandler::memberships
pub struct TeamMembershipBuilder<'octo> {
    crab: &'octo Octocrab,
    org: String,
    team: String,
}

impl<'octo> TeamMembershipBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String, team: String) -> Self {
        Self { crab, org, team }
    }

    /// Gets a user's membership in the team.
    ///
    /// Team members will include the members of child teams.
    /// To get a user's membership with a team, the team must be visible to the authenticated user.
    ///
    /// See <https://docs.github.com/en/rest/teams/members?apiVersion=2026-03-10#get-team-membership-for-a-user>
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let membership = octocrab::instance()
    ///     .teams("owner")
    ///     .memberships("team")
    ///     .get("username")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, username: impl Into<String>) -> Result<TeamMembership> {
        let route = format!(
            "/orgs/{org}/teams/{team}/memberships/{username}",
            org = self.org,
            team = self.team,
            username = username.into(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Adds or updates a user's membership in the team.
    ///
    /// Adds an organization member to a team or updates their role within the team.
    /// If the user is unaffiliated with the organization, this sends an invitation email
    /// and their membership will be in the `pending` state until accepted.
    ///
    /// See <https://docs.github.com/en/rest/teams/members?apiVersion=2026-03-10#add-or-update-team-membership-for-a-user>
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::models::memberships::Role;
    ///
    /// let membership = octocrab::instance()
    ///     .teams("owner")
    ///     .memberships("team")
    ///     .add_or_update("username", Some(Role::Maintainer))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_or_update(
        &self,
        username: impl Into<String>,
        role: impl Into<Option<Role>>,
    ) -> Result<TeamMembership> {
        let route = format!(
            "/orgs/{org}/teams/{team}/memberships/{username}",
            org = self.org,
            team = self.team,
            username = username.into(),
        );
        let role_body = role.into().map(|role| RoleUpdateBody { role });
        self.crab.put(route, role_body.as_ref()).await
    }

    /// Removes a user's membership from the team.
    ///
    /// Removing team membership does not delete the user, it just removes their membership
    /// from the team.
    ///
    /// See <https://docs.github.com/en/rest/teams/members?apiVersion=2026-03-10#remove-team-membership-for-a-user>
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .memberships("team")
    ///     .remove("username")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove(&self, username: impl Into<String>) -> Result<()> {
        let route = format!(
            "/orgs/{org}/teams/{team}/memberships/{username}",
            org = self.org,
            team = self.team,
            username = username.into(),
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }
}
