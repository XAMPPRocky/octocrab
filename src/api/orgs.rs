//! The Organization API.

mod code_scanning;
mod copilot;
mod copilot_seat_manager;
mod custom_properties;
mod events;
mod hooks;
mod invitations;
mod issues;
mod list_members;
mod list_repos;
mod personal_access_tokens;
mod roles;
mod rulesets;
mod secret_scanning_alerts;
mod secrets;

pub use self::code_scanning::{ListOrgCodeScanningAlertsBuilder, OrgCodeScanningHandler};
pub use self::custom_properties::{ListOrgCustomPropertyValuesBuilder, OrgCustomPropertiesHandler};
pub use self::events::ListOrgEventsBuilder;
pub use self::hooks::{ListOrgHooksBuilder, OrgHooksHandler, UpdateOrgHookBuilder};
pub use self::invitations::{
    CreateOrgInvitationBuilder, ListFailedOrgInvitationsBuilder, ListOrgInvitationTeamsBuilder,
    ListOrgInvitationsBuilder, OrgInvitationsHandler,
};
pub use self::issues::ListOrgIssuesBuilder;
pub use self::list_members::ListOrgMembersBuilder;
pub use self::list_repos::ListReposBuilder;
pub use self::personal_access_tokens::{
    ListOrgPatRepositoriesBuilder, ListOrgPatRequestRepositoriesBuilder, ListOrgPatRequestsBuilder,
    ListOrgPersonalAccessTokensBuilder, OrgPersonalAccessTokensHandler,
};
pub use self::roles::OrgRolesHandler;
pub use self::rulesets::{
    ListOrgRuleSuitesBuilder, ListOrgRulesetsBuilder, OrgRuleSuitesHandler, OrgRulesetsHandler,
};
pub use self::secret_scanning_alerts::OrgSecretScanningAlertsHandler;
pub use self::secrets::OrgSecretsHandler;
pub use crate::api::security_advisories::OrgSecurityAdvisoriesHandler;
use crate::error::HttpSnafu;
use crate::models::interaction_limits;
use crate::models::interaction_limits::InteractionLimit;
use crate::models::{Author, Installation};
use crate::Octocrab;
use crate::Page;
use http::{StatusCode, Uri};
use interaction_limits::{InteractionLimitExpiry, InteractionLimitType};
use snafu::ResultExt;

/// A client to GitHub's organization API.
///
/// Created with [`Octocrab::orgs`].
pub struct OrgHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String) -> Self {
        Self { crab, owner }
    }

    /// Handle packages for the organization.
    ///
    /// See: https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28
    pub fn packages(&self) -> crate::api::packages::PackagesHandler<'octo> {
        crate::api::packages::PackagesHandler::new(
            self.crab,
            crate::api::packages::PackagesOwner::Org(self.owner.clone()),
        )
    }

    /// Handle security advisories for the organization.
    ///
    /// See: https://docs.github.com/en/rest/security-advisories/repository-advisories#list-repository-security-advisories-for-an-organization
    pub fn security_advisories(&self) -> OrgSecurityAdvisoriesHandler<'octo> {
        OrgSecurityAdvisoriesHandler::new(self.crab, self.owner.clone())
    }

    /// Handle secret scanning alerts for the organization.
    ///
    /// See: https://docs.github.com/en/rest/secret-scanning?apiVersion=2022-11-28#list-secret-scanning-alerts-for-an-organization
    pub fn secret_scanning(&self) -> OrgSecretScanningAlertsHandler<'_> {
        OrgSecretScanningAlertsHandler::new(self)
    }

    /// Handle secret scanning alerts for the organization (alias for [`secret_scanning`][OrgHandler::secret_scanning]).
    pub fn secrets_scanning(&self) -> OrgSecretScanningAlertsHandler<'_> {
        self.secret_scanning()
    }

    /// Handle code scanning alerts for the organization.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-an-organization
    pub fn code_scanning(&self) -> OrgCodeScanningHandler<'octo, '_> {
        OrgCodeScanningHandler::new(self)
    }

    /// Handle code scanning alerts for the organization (alias for [`code_scanning`][OrgHandler::code_scanning]).
    pub fn code_scannings(&self) -> OrgCodeScanningHandler<'octo, '_> {
        self.code_scanning()
    }

    /// Handle rulesets for the organization.
    ///
    /// See: https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28
    pub fn rulesets(&self) -> OrgRulesetsHandler<'octo, '_> {
        OrgRulesetsHandler::new(self)
    }

    /// Handle webhooks for the organization.
    ///
    /// See: https://docs.github.com/en/rest/orgs/webhooks?apiVersion=2022-11-28
    pub fn hooks(&self) -> OrgHooksHandler<'octo, '_> {
        OrgHooksHandler::new(self)
    }

    /// Handle custom properties for the organization.
    ///
    /// See: https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28
    pub fn custom_properties(&self) -> OrgCustomPropertiesHandler<'octo, '_> {
        OrgCustomPropertiesHandler::new(self)
    }

    /// Handle organization roles.
    ///
    /// See: https://docs.github.com/en/rest/orgs/organization-roles?apiVersion=2022-11-28
    pub fn roles(&self) -> OrgRolesHandler<'octo, '_> {
        OrgRolesHandler::new(self)
    }

    /// Handle invitations for the organization.
    ///
    /// See: https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28
    pub fn invitations(&self) -> OrgInvitationsHandler<'octo, '_> {
        OrgInvitationsHandler::new(self)
    }

    /// Handle fine-grained personal access tokens for the organization.
    ///
    /// See: https://docs.github.com/en/rest/orgs/personal-access-tokens?apiVersion=2022-11-28
    pub fn personal_access_tokens(&self) -> OrgPersonalAccessTokensHandler<'octo, '_> {
        OrgPersonalAccessTokensHandler::new(self)
    }

    /// List organization issues assigned to the authenticated user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-organization-issues-assigned-to-the-authenticated-user)
    pub fn list_issues(&self) -> ListOrgIssuesBuilder<'octo, '_, '_> {
        ListOrgIssuesBuilder::new(self)
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
        let route = format!(
            "/orgs/{org}/memberships/{username}",
            org = self.owner,
            username = username.as_ref(),
        );

        let body = role.map(|role| serde_json::json!({ "role": role }));

        self.crab.post(route, body.as_ref()).await
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
        let route = format!(
            "/orgs/{org}/members/{username}",
            org = self.owner,
            username = username.as_ref(),
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        let response = self.crab._get(uri).await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(crate::map_github_error(response).await.unwrap_err()),
        }
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
    pub fn list_repos(&self) -> list_repos::ListReposBuilder<'_, '_> {
        list_repos::ListReposBuilder::new(self)
    }

    /// List events on this organization.
    ///
    /// Takes an optional etag which allows for efficient polling. Here is a quick example to poll a
    /// organization's events.
    /// ```no_run
    /// # use std::convert::TryFrom;
    /// # use octocrab::{models::events::Event, etag::{Etagged,EntityTag}, Page};
    /// # async fn run() -> octocrab::Result<()> {
    /// let mut etag = None;
    /// loop {
    ///     let response: Etagged<Page<Event>> = octocrab::instance()
    ///         .orgs("owner")
    ///         .events()
    ///         .etag(etag)
    ///         .send()
    ///         .await?;
    ///     if let Some(page) = response.value {
    ///         // do something with the page ...
    ///     } else {
    ///         println!("No new data received, trying again soon");
    ///     }
    ///     etag = response.etag;
    ///     // add a delay before the next iteration
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn events(&self) -> events::ListOrgEventsBuilder<'_, '_> {
        events::ListOrgEventsBuilder::new(self)
    }

    /// Creates a new webhook for the specified organization.
    ///
    /// # Notes
    /// Only authorized users or apps can modify organization webhooks.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models::hooks::{Hook, Config as HookConfig, ContentType as HookContentType};
    ///
    /// let config = HookConfig {
    ///   url: "https://example.com".to_string(),
    ///   content_type: Some(HookContentType::Json),
    ///   insecure_ssl: None,
    ///   secret: None
    /// };
    ///
    /// let hook = Hook {
    ///   name: "web".to_string(),
    ///   config,
    ///   ..Hook::default()
    /// };
    ///
    /// let hook = octocrab.orgs("owner").create_hook(hook).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_hook(
        &self,
        hook: crate::models::hooks::Hook,
    ) -> crate::Result<crate::models::hooks::Hook> {
        let route = format!("/orgs/{org}/hooks", org = self.owner);
        let res = self.crab.post(route, Some(&hook)).await?;

        Ok(res)
    }

    /// Lists members of the specified organization.
    ///
    /// # Notes
    /// Only authorized users who belong to the organization can list its members.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let org_members = octocrab::instance().orgs("org").list_members().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_members(&self) -> list_members::ListOrgMembersBuilder<'_, '_> {
        list_members::ListOrgMembersBuilder::new(self)
    }

    /// Handle secrets on the organizaton
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let octocrab = octocrab::instance();
    /// let secrets = octocrab.orgs("org").secrets();
    /// # Ok(())
    /// # }
    /// ```
    pub fn secrets(&self) -> secrets::OrgSecretsHandler<'_> {
        secrets::OrgSecretsHandler::new(self)
    }

    /// ### Get interaction restrictions for an organization
    ///
    /// Shows which type of GitHub user can interact with this organization and when the restriction expires. If there is no restrictions, you will see an empty response.
    ///
    /// Fine-grained access tokens for "Get interaction restrictions for an organization"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - GitHub App installation access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// - "Administration" organization permissions (read)
    ///
    pub async fn get_interaction_restrictions(
        &self,
    ) -> crate::Result<interaction_limits::InteractionLimit> {
        let route = format!("/orgs/{}/interaction-limits", self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// ### Set interaction restrictions for an organization
    ///
    /// Temporarily restricts interactions to a certain type of GitHub user in any public repository in the given organization. You must be an organization owner to set these restrictions. Setting the interaction limit at the organization level will overwrite any interaction limits that are set for individual repositories owned by the organization.
    ///
    /// Fine-grained access tokens for "Set interaction restrictions for an organization"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - GitHub App installation access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// - "Administration" organization permissions (write)
    ///
    pub async fn set_interaction_restrictions(
        &self,
        limit_type: InteractionLimitType,
        expiry: InteractionLimitExpiry,
    ) -> crate::Result<InteractionLimit> {
        let route = format!("/orgs/{}/interaction-limits", self.owner);
        let body = serde_json::json!({
            "limit": limit_type,
            "expiry": expiry,
        });
        self.crab.put(route, Some(&body)).await
    }

    /// Handle copilot-related calls on the organization
    ///
    /// # Examples
    /// ```no_run
    /// async fn run() {
    ///     let copilot_usage = octocrab::instance().orgs("org").copilot().metrics().await.expect("failed to retrieve usage");
    /// }
    /// ```
    pub fn copilot(&self) -> copilot::CopilotHandler<'octo, '_> {
        copilot::CopilotHandler::new(self)
    }

    /// ### Remove interaction restrictions for an organization
    ///
    /// Removes all interaction restrictions from public repositories in the given organization. You must be an organization owner to remove restrictions.
    ///
    /// Fine-grained access tokens for "Remove interaction restrictions for an organization"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - GitHub App installation access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// - "Administration" organization permissions (write)
    ///
    /// ```no_run
    ///  async fn run() -> crate::octocrab::Result<()> {
    ///   let org_name = "example-org".to_string();
    ///   let crab = octocrab::instance();
    ///   crab.orgs(org_name).remove_interaction_restrictions().await?;
    ///   Ok(())
    ///  }
    /// ```
    pub async fn remove_interaction_restrictions(&self) -> crate::Result<()> {
        let route = format!("/orgs/{}/interaction-limits", self.owner);
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Lists public members of an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#list-public-organization-members)
    pub fn list_public_members(&self) -> ListPublicMembersBuilder<'octo, '_> {
        ListPublicMembersBuilder::new(self)
    }

    /// Checks if a user is a public member of an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#check-public-organization-membership-for-a-user)
    pub async fn check_public_membership(&self, username: impl AsRef<str>) -> crate::Result<bool> {
        let route = format!(
            "/orgs/{org}/public_members/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let response = self.crab._get(uri).await?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(crate::map_github_error(response).await.unwrap_err()),
        }
    }

    /// Sets public organization membership for the authenticated user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#set-public-organization-membership-for-the-authenticated-user)
    pub async fn publicize_membership(&self, username: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/public_members/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let response = self.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Removes public organization membership for the authenticated user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#remove-public-organization-membership-for-the-authenticated-user)
    pub async fn conceal_membership(&self, username: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/public_members/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists outside collaborators for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/outside-collaborators?apiVersion=2022-11-28#list-outside-collaborators-for-an-organization)
    pub fn list_outside_collaborators(&self) -> ListOutsideCollaboratorsBuilder<'octo, '_> {
        ListOutsideCollaboratorsBuilder::new(self)
    }

    /// Converts an organization member to outside collaborator.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/outside-collaborators?apiVersion=2022-11-28#convert-an-organization-member-to-outside-collaborator)
    pub async fn convert_to_outside_collaborator(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/outside_collaborators/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let response = self.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Removes an outside collaborator from an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/outside-collaborators?apiVersion=2022-11-28#remove-outside-collaborator-from-an-organization)
    pub async fn remove_outside_collaborator(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/outside_collaborators/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists users blocked by an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/blocking?apiVersion=2022-11-28#list-users-blocked-by-an-organization)
    pub fn list_blocked_users(&self) -> ListBlockedUsersBuilder<'octo, '_> {
        ListBlockedUsersBuilder::new(self)
    }

    /// Checks if a user is blocked by an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/blocking?apiVersion=2022-11-28#check-if-a-user-is-blocked-by-an-organization)
    pub async fn check_blocked_user(&self, username: impl AsRef<str>) -> crate::Result<bool> {
        let route = format!(
            "/orgs/{org}/blocks/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let response = self.crab._get(uri).await?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(crate::map_github_error(response).await.unwrap_err()),
        }
    }

    /// Blocks a user from an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/blocking?apiVersion=2022-11-28#block-a-user-from-an-organization)
    pub async fn block_user(&self, username: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/blocks/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let response = self.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Unblocks a user from an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/blocking?apiVersion=2022-11-28#unblock-a-user-from-an-organization)
    pub async fn unblock_user(&self, username: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/blocks/{username}",
            org = self.owner,
            username = username.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists teams that are security managers for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/security-managers?apiVersion=2022-11-28#list-security-manager-teams)
    pub async fn list_security_managers(&self) -> crate::Result<Vec<crate::models::teams::Team>> {
        let route = format!("/orgs/{org}/security-managers", org = self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// Adds a security manager team to an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/security-managers?apiVersion=2022-11-28#add-a-security-manager-team)
    pub async fn add_security_manager_team(&self, team_slug: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/security-managers/teams/{team_slug}",
            org = self.owner,
            team_slug = team_slug.as_ref()
        );
        let response = self.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Removes a security manager team from an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/security-managers?apiVersion=2022-11-28#remove-a-security-manager-team)
    pub async fn remove_security_manager_team(
        &self,
        team_slug: impl AsRef<str>,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/security-managers/teams/{team_slug}",
            org = self.owner,
            team_slug = team_slug.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Enables or disables a security feature for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/orgs?apiVersion=2022-11-28#enable-or-disable-a-security-feature-for-an-organization)
    pub async fn set_security_product_enablement(
        &self,
        product: crate::models::orgs::security::SecurityProduct,
        enablement: crate::models::orgs::security::SecurityEnablement,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/{product}/{enablement}",
            org = self.owner,
            product = product,
            enablement = enablement
        );
        let response = self.crab._patch(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists app installations for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/orgs?apiVersion=2022-11-28#list-app-installations-for-an-organization)
    pub fn list_installations(&self) -> ListOrgInstallationsBuilder<'octo, '_> {
        ListOrgInstallationsBuilder::new(self)
    }
}

/// Builder for listing public organization members.
#[derive(serde::Serialize)]
pub struct ListPublicMembersBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListPublicMembersBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> crate::Result<Page<Author>> {
        let route = format!("/orgs/{org}/public_members", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing outside collaborators for an organization.
#[derive(serde::Serialize)]
pub struct ListOutsideCollaboratorsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOutsideCollaboratorsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            filter: None,
            per_page: None,
            page: None,
        }
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> crate::Result<Page<Author>> {
        let route = format!(
            "/orgs/{org}/outside_collaborators",
            org = self.handler.owner
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing users blocked by an organization.
#[derive(serde::Serialize)]
pub struct ListBlockedUsersBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListBlockedUsersBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> crate::Result<Page<Author>> {
        let route = format!("/orgs/{org}/blocks", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing app installations for an organization.
#[derive(serde::Serialize)]
pub struct ListOrgInstallationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgInstallationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> crate::Result<Page<Installation>> {
        let route = format!("/orgs/{org}/installations", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}
