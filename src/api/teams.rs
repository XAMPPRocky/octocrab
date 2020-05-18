//! The Teams API

mod children;
mod create;
mod edit;
mod list;
mod team_repos;

pub use {
    children::ListChildTeamsBuilder,
    create::CreateTeamBuilder,
    edit::EditTeamBuilder,
    list::ListTeamsBuilder,
    team_repos::{ManagesRepo, TeamRepoHandler},
};

use crate::{models, Octocrab, Result};

pub struct TeamHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> TeamHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String) -> Self {
        Self { crab, owner }
    }

    pub fn list(&self) -> ListTeamsBuilder<'_, '_> {
        ListTeamsBuilder::new(self)
    }

    pub async fn get(&self, team_slug: impl Into<String>) -> Result<models::Team> {
        let url = format!(
            "/orgs/{org}/teams/{team}",
            org = self.owner,
            team = team_slug.into(),
        );
        self.crab.get(url, None::<&()>).await
    }

    pub fn create(&self, name: impl Into<String>) -> CreateTeamBuilder {
        CreateTeamBuilder::new(self, name.into())
    }

    pub fn edit(&self, team_slug: impl Into<String>, name: impl Into<String>) -> EditTeamBuilder {
        EditTeamBuilder::new(self, team_slug.into(), name.into())
    }

    pub async fn delete(&self, team_slug: impl Into<String>) -> Result<()> {
        let url = format!(
            "/orgs/{org}/teams/{team}",
            org = self.owner,
            team = team_slug.into(),
        );
        self.crab.delete(url, None::<&()>).await
    }

    pub fn children(&self, team_slug: impl Into<String>) -> ListChildTeamsBuilder {
        ListChildTeamsBuilder::new(self, team_slug.into())
    }

    pub fn repos(&self, team_slug: impl Into<String>) -> TeamRepoHandler {
        TeamRepoHandler::new(self.crab, self.owner.clone(), team_slug.into())
    }
}
