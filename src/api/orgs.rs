//! The Organization API.

mod list_repos;

use crate::Octocrab;

pub use self::list_repos::ListReposBuilder;

/// A client to GitHub's organization API.
///
/// Created with [`Octocrab::orgs`].
///
/// [`Octocrab::orgs`]: ../struct.Octocrab.html#method.orgs
pub struct OrgHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String) -> Self {
        Self { crab, owner }
    }

    /// Add or update organization membership
    ///
    /// **Note**
    /// - Only authenticated organization owners can add a member to the
    ///   organization or update the member's role.
    /// - If the authenticated user is adding a member to the organization, the
    ///   invited user will receive an email inviting them to the organization.
    ///   The user's membership status will be pending until they accept
    ///   the invitation.
    /// - Authenticated users can update a user's membership by passing the role
    ///   parameter. If the authenticated user changes a member's role to admin,
    ///   the affected user will receive an email notifying them that they've
    ///   been made an organization owner. If the authenticated user changes an
    ///   owner's role to member, no email will be sent.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let invitation = octocrab.orgs("owner").add_or_update_membership("ferris", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_or_update_membership(
        &self,
        username: impl AsRef<str>,
        role: Option<crate::params::orgs::Role>,
    ) -> crate::Result<crate::models::orgs::MembershipInvitation> {
        let url = format!(
            "/orgs/{org}/memberships/{username}",
            org = self.owner,
            username = username.as_ref(),
        );

        let body = role.map(|role| serde_json::json!({ "role": role }));

        self.crab.post(url, body.as_ref()).await
    }

    /// Check if a user is, publicly or privately, a member of the organization.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// assert!(octocrab.orgs("owner").check_membership("ferris").await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_membership(&self, username: impl AsRef<str>) -> crate::Result<bool> {
        let url = format!(
            "/orgs/{org}/members/{username}",
            org = self.owner,
            username = username.as_ref(),
        );

        let response = self
            .crab
            ._get(self.crab.absolute_url(url)?, None::<&()>)
            .await?;
        let status = response.status();

        Ok(status == 204 || status == 301)
    }

    /// Get an organization
    ///
    /// To see many of the organization response values, you need to be an
    /// authenticated organization owner with the `admin:org` scope. When the
    /// value of `two_factor_requirement_enabled` is true, the organization
    /// requires all members, billing managers, and outside collaborators to
    /// enable two-factor authentication.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.orgs("owner").get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> crate::Result<crate::models::orgs::Organization> {
        let route = format!("/orgs/{org}", org = self.owner);

        self.crab.get(route, None::<&()>).await
    }

    /// List repos for the specified organization.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// // Get the least active repos belonging to `owner`.
    /// let page = octocrab::instance()
    ///     .orgs("owner")
    ///     .list_repos()
    ///     // Optional Parameters
    ///     .repo_type(params::repos::Type::Sources)
    ///     .sort(params::repos::Sort::Pushed)
    ///     .direction(params::Direction::Descending)
    ///     .per_page(25)
    ///     .page(5u32)
    ///     // Send the request.
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_repos(&self) -> list_repos::ListReposBuilder {
        list_repos::ListReposBuilder::new(self)
    }
}
