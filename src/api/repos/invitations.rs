//! GitHub Repository Invitations API.
//!
//! See: [GitHub API Documentation](https://docs.github.com/en/rest/collaborators/invitations?apiVersion=2022-11-28)

use super::RepoHandler;
use crate::{
    models::{
        repos::{InvitationPermission, RepositoryInvitation},
        InvitationId,
    },
    Page, Result,
};

/// A client to GitHub's repository invitations API.
///
/// Created with [`RepoHandler::invitations`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/collaborators/invitations?apiVersion=2022-11-28)
pub struct RepoInvitationsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoInvitationsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`ListRepoInvitationsBuilder`] to list all open invitations for the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/collaborators/invitations?apiVersion=2022-11-28#list-repository-invitations)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let invitations = octocrab
    ///     .repos("owner", "repo")
    ///     .invitations()
    ///     .list()
    ///     .per_page(100)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListRepoInvitationsBuilder<'octo, 'r> {
        ListRepoInvitationsBuilder::new(self.handler)
    }

    /// Creates an [`UpdateRepoInvitationBuilder`] to update permissions for an existing invitation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/collaborators/invitations?apiVersion=2022-11-28#update-a-repository-invitation)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::repos::InvitationPermission;
    ///
    /// let updated = octocrab
    ///     .repos("owner", "repo")
    ///     .invitations()
    ///     .update(12345u64)
    ///     .permissions(InvitationPermission::Write)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &self,
        invitation_id: impl Into<InvitationId>,
    ) -> UpdateRepoInvitationBuilder<'octo, 'r> {
        UpdateRepoInvitationBuilder::new(self.handler, invitation_id.into())
    }

    /// Deletes a repository invitation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/collaborators/invitations?apiVersion=2022-11-28#delete-a-repository-invitation)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .invitations()
    ///     .delete(12345u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, invitation_id: impl Into<InvitationId>) -> Result<()> {
        let route = format!(
            "/{}/invitations/{}",
            self.handler.repo,
            invitation_id.into()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for listing open repository invitations.
///
/// Created by [`RepoInvitationsHandler::list`] or [`RepoHandler::list_invitations`].
#[derive(serde::Serialize)]
pub struct ListRepoInvitationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRepoInvitationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100). Default: 30.
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<RepositoryInvitation>> {
        let route = format!("/{}/invitations", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating permissions on a repository invitation.
///
/// Created by [`RepoInvitationsHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdateRepoInvitationBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    invitation_id: InvitationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<InvitationPermission>,
}

impl<'octo, 'r> UpdateRepoInvitationBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, invitation_id: InvitationId) -> Self {
        Self {
            handler,
            invitation_id,
            permissions: None,
        }
    }

    /// The permissions that the associated user will have on the repository.
    ///
    /// Valid values are `read`, `write`, `maintain`, `triage`, and `admin`.
    pub fn permissions(mut self, permissions: impl Into<InvitationPermission>) -> Self {
        self.permissions = Some(permissions.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<RepositoryInvitation> {
        let route = format!("/{}/invitations/{}", self.handler.repo, self.invitation_id);
        self.handler.crab.patch(route, Some(&self)).await
    }
}
