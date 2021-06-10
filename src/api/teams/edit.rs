use super::*;
use crate::params;
use crate::models::TeamId;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct EditTeamBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamHandler<'octo>,
    #[serde(skip)]
    slug: String,
    name: String,
    /// The description of the team.
    description: Option<String>,
    /// The level of privacy this team should have.
    ///
    /// For parents or child teams, only `Privacy::Closed` is valid.
    privacy: Option<params::teams::Privacy>,
    permission: Option<params::teams::Permission>,
    /// The ID of the team to set as the parent team.
    parent_team_id: Option<TeamId>,
}

impl<'octo, 'r> EditTeamBuilder<'octo, 'r> {
    /// Sends the actual request.
    pub async fn send(self) -> Result<models::teams::Team> {
        let url = format!(
            "orgs/{org}/teams/{team}",
            org = self.handler.owner,
            team = self.slug,
        );
        self.handler.crab.patch(url, Some(&self)).await
    }
}
