use super::*;
use crate::params;

#[derive(serde::Serialize)]
pub struct CreateTeamBuilder<'octo, 'a> {
    #[serde(skip)]
    handler: &'a TeamHandler<'octo>,
    name: String,
    description: Option<String>,
    maintainers: Option<Vec<String>>,
    repo_names: Option<Vec<String>>,
    privacy: Option<params::teams::Privacy>,
    permission: Option<params::teams::Permission>,
    parent_team_id: Option<i64>,
}

impl<'octo, 'a> CreateTeamBuilder<'octo, 'a> {
    pub(crate) fn new(handler: &'a TeamHandler<'octo>, name: String) -> Self {
        Self {
            handler,
            name,
            description: None,
            maintainers: None,
            repo_names: None,
            privacy: None,
            permission: None,
            parent_team_id: None,
        }
    }

    /// The description of the team.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The organization members who will become team maintainers.
    pub fn maintainers(mut self, maintainers: impl Into<Vec<String>>) -> Self {
        self.maintainers = Some(maintainers.into());
        self
    }

    /// The repositories to add the team to.
    ///
    /// Note: the repo name must be its full name, e.g. `"org/repo"`.
    pub fn repo_names(mut self, repo_names: impl Into<Vec<String>>) -> Self {
        self.repo_names = Some(repo_names.into());
        self
    }

    /// The level of privacy this team should have.
    ///
    /// For parents or child teams, only `Privacy::Closed` is valid.
    pub fn privacy(mut self, privacy: impl Into<params::teams::Privacy>) -> Self {
        self.privacy = Some(privacy.into());
        self
    }

    /// The ID of the team to set as the parent team.
    pub fn parent_team_id(mut self, parent_team_id: impl Into<i64>) -> Self {
        self.parent_team_id = Some(parent_team_id.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::Team> {
        let url = format!("/orgs/{org}/teams", org = self.handler.owner,);
        self.handler.crab.post(url, Some(&self)).await
    }
}
