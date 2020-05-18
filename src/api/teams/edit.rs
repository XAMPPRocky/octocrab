use super::*;
use crate::params;

#[derive(serde::Serialize)]
pub struct EditTeamBuilder<'octo, 'a> {
    #[serde(skip)]
    handler: &'a TeamHandler<'octo>,
    #[serde(skip)]
    slug: String,
    name: String,
    description: Option<String>,
    privacy: Option<params::teams::Privacy>,
    permission: Option<params::teams::Permission>,
    parent_team_id: Option<i64>,
}

impl<'octo, 'a> EditTeamBuilder<'octo, 'a> {
    pub(crate) fn new(handler: &'a TeamHandler<'octo>, slug: String, name: String) -> Self {
        Self {
            handler,
            slug,
            name,
            description: None,
            privacy: None,
            permission: None,
            parent_team_id: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn privacy(mut self, privacy: impl Into<params::teams::Privacy>) -> Self {
        self.privacy = Some(privacy.into());
        self
    }

    pub fn parent_team_id(mut self, parent_team_id: impl Into<i64>) -> Self {
        self.parent_team_id = Some(parent_team_id.into());
        self
    }

    pub async fn send(self) -> Result<models::Team> {
        let url = format!(
            "/orgs/{org}/teams/{team}",
            org = self.handler.owner,
            team = self.slug,
        );
        self.handler.crab.patch(url, Some(&self)).await
    }
}
