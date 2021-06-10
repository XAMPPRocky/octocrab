use super::*;
use crate::params;
use crate::models::TeamId;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct CreateTeamBuilder<'octo, 'h, 'a, 'b> {
    #[serde(skip)]
    handler: &'h TeamHandler<'octo>,
    name: String,
    /// The description of the team.
    description: Option<String>,
    /// The organization members who will become team maintainers.
    maintainers: Option<&'a [String]>,
    /// The repositories to add the team to.
    ///
    /// Note: the repo name must be its full name, e.g. `"org/repo"`.
    repo_names: Option<&'b [String]>,
    /// The level of privacy this team should have.
    ///
    /// For parents or child teams, only `Privacy::Closed` is valid.
    privacy: Option<params::teams::Privacy>,
    permission: Option<params::teams::Permission>,
    /// The ID of the team to set as the parent team.
    parent_team_id: Option<TeamId>,
}

impl<'octo, 'h, 'a, 'b> CreateTeamBuilder<'octo, 'h, 'a, 'b> {
    /// Sends the actual request.
    pub async fn send(self) -> Result<models::teams::Team> {
        let url = format!("orgs/{org}/teams", org = self.handler.owner,);
        self.handler.crab.post(url, Some(&self)).await
    }
}
