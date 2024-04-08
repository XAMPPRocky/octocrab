//! The Organization API.

mod events;
mod list_members;
mod list_repos;
mod secrets;

use crate::error::HttpSnafu;
use crate::Octocrab;
use http::{StatusCode, Uri};
use snafu::ResultExt;

pub use self::events::ListOrgEventsBuilder;
pub use self::list_members::ListOrgMembersBuilder;
pub use self::list_repos::ListReposBuilder;
pub use self::secrets::OrgSecretsHandler;

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
    pub fn list_repos(&self) -> list_repos::ListReposBuilder {
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
    pub fn list_members(&self) -> list_members::ListOrgMembersBuilder {
        list_members::ListOrgMembersBuilder::new(self)
    }

    /// Handle secrets on the organizaton
    /// ```no_run
    /// ```
    pub fn secrets(&self) -> secrets::OrgSecretsHandler<'_> {
        secrets::OrgSecretsHandler::new(self)
    }
}
