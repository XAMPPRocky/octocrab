use super::OrgHandler;
use crate::models::orgs::invitations::{FailedOrgInvitation, OrgInvitation};
use crate::models::teams::Team;
use crate::models::InvitationId;
use crate::{Page, Result};

/// A client to GitHub's organization invitations API.
///
/// Created with [`OrgHandler::invitations`].
pub struct OrgInvitationsHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgInvitationsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists pending invitations for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#list-pending-organization-invitations)
    pub fn list(&self) -> ListOrgInvitationsBuilder<'octo, 'r> {
        ListOrgInvitationsBuilder::new(self.handler)
    }

    /// Creates an organization invitation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#create-an-organization-invitation)
    pub fn create(&self) -> CreateOrgInvitationBuilder<'octo, 'r> {
        CreateOrgInvitationBuilder::new(self.handler)
    }

    /// Cancels an organization invitation.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#cancel-an-organization-invitation)
    pub async fn cancel(&self, invitation_id: impl Into<InvitationId>) -> Result<()> {
        let invitation_id = invitation_id.into();
        let route = format!(
            "/orgs/{org}/invitations/{invitation_id}",
            org = self.handler.owner
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists organization invitation teams.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#list-organization-invitation-teams)
    pub fn list_teams(
        &self,
        invitation_id: impl Into<InvitationId>,
    ) -> ListOrgInvitationTeamsBuilder<'octo, 'r> {
        ListOrgInvitationTeamsBuilder::new(self.handler, invitation_id.into())
    }

    /// Lists failed organization invitations.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/members?apiVersion=2022-11-28#list-failed-organization-invitations)
    pub fn list_failed(&self) -> ListFailedOrgInvitationsBuilder<'octo, 'r> {
        ListFailedOrgInvitationsBuilder::new(self.handler)
    }
}

/// Builder for listing organization invitations.
#[derive(serde::Serialize)]
pub struct ListOrgInvitationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitation_source: Option<String>,
}

impl<'octo, 'r> ListOrgInvitationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            role: None,
            invitation_source: None,
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

    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn invitation_source(mut self, source: impl Into<String>) -> Self {
        self.invitation_source = Some(source.into());
        self
    }

    pub async fn send(self) -> Result<Page<OrgInvitation>> {
        let route = format!("/orgs/{org}/invitations", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for creating an organization invitation.
#[derive(serde::Serialize, Default)]
pub struct CreateOrgInvitationBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: Option<&'r OrgHandler<'octo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitee_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_ids: Option<Vec<u64>>,
}

impl<'octo, 'r> CreateOrgInvitationBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler: Some(handler),
            invitee_id: None,
            email: None,
            role: None,
            team_ids: None,
        }
    }

    pub fn invitee_id(mut self, invitee_id: impl Into<u64>) -> Self {
        self.invitee_id = Some(invitee_id.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn team_ids(mut self, team_ids: impl Into<Vec<u64>>) -> Self {
        self.team_ids = Some(team_ids.into());
        self
    }

    pub async fn send(self) -> Result<OrgInvitation> {
        let handler = self.handler.expect("handler must be present");
        let route = format!("/orgs/{org}/invitations", org = handler.owner);
        handler.crab.post(route, Some(&self)).await
    }
}

/// Builder for listing teams in an organization invitation.
#[derive(serde::Serialize)]
pub struct ListOrgInvitationTeamsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip)]
    invitation_id: InvitationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgInvitationTeamsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>, invitation_id: InvitationId) -> Self {
        Self {
            handler,
            invitation_id,
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

    pub async fn send(self) -> Result<Page<Team>> {
        let route = format!(
            "/orgs/{org}/invitations/{invitation_id}/teams",
            org = self.handler.owner,
            invitation_id = self.invitation_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing failed organization invitations.
#[derive(serde::Serialize)]
pub struct ListFailedOrgInvitationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListFailedOrgInvitationsBuilder<'octo, 'r> {
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

    pub async fn send(self) -> Result<Page<FailedOrgInvitation>> {
        let route = format!("/orgs/{org}/failed_invitations", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}
