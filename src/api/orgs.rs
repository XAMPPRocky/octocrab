use crate::Octocrab;

/// A client to GitHub's organization API.
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
    /// let pr = octocrab.orgs("owner").add_or_update_membership("ferris", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_or_update_membership(
        &self,
        username: impl AsRef<str>,
        role: Option<crate::params::orgs::Role>,
    ) -> crate::Result<crate::models::MembershipInvitation> {
        let url = format!(
            "/orgs/{org}/memberships/{username}",
            org = self.owner,
            username = username.as_ref(),
        );

        let body = role.map(|role| serde_json::json!({ "role": role }));

        self.crab.post(url, body.as_ref()).await
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
    /// let pr = octocrab.orgs("owner").get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> crate::Result<crate::models::Organization> {
        let route = format!("/orgs/{org}", org = self.owner);

        self.crab.get(route, None::<&()>).await
    }
}
