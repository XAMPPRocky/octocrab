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

/// Handler for GitHub's teams API.
///
/// Created with [`Octocrab::teams`].
///
/// [`Octocrab::teams`]: ../struct.Octocrab.html#method.teams
pub struct TeamHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> TeamHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String) -> Self {
        Self { crab, owner }
    }

    /// Lists teams in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let teams = octocrab.teams("owner")
    ///     .list()
    ///     .per_page(10)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListTeamsBuilder<'_, '_> {
        ListTeamsBuilder::new(self)
    }

    /// Gets a team from its slug.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let team = octocrab.teams("owner").get("team").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, team_slug: impl Into<String>) -> Result<models::Team> {
        let url = format!(
            "/orgs/{org}/teams/{team}",
            org = self.owner,
            team = team_slug.into(),
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Creates a new team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// octocrab.teams("owner")
    ///     .create("new-team")
    ///     .description("My team created from Octocrab!")
    ///     .maintainers(&["ferris"])
    ///     .repo_names(&["crab-stuff"])
    ///     .privacy(params::teams::Privacy::Closed)
    ///     .parent_team_id(1)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, name: impl Into<String>) -> CreateTeamBuilder {
        CreateTeamBuilder::new(self, name.into())
    }

    /// Creates a new team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// octocrab.teams("owner")
    ///     .edit("some-team", "Some Team")
    ///     .description("I edited from Octocrab!")
    ///     .privacy(params::teams::Privacy::Secret)
    ///     .parent_team_id(2)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit(&self, team_slug: impl Into<String>, name: impl Into<String>) -> EditTeamBuilder {
        EditTeamBuilder::new(self, team_slug.into(), name.into())
    }

    /// Deletes a team from the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.teams("owner").delete("some-team").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, team_slug: impl Into<String>) -> Result<()> {
        let url = format!(
            "/orgs/{org}/teams/{team}",
            org = self.owner,
            team = team_slug.into(),
        );
        self.crab.delete(url, None::<&()>).await
    }

    /// List the child teams of a team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.teams("owner")
    ///     .children("parent-team")
    ///     .per_page(5)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn children(&self, team_slug: impl Into<String>) -> ListChildTeamsBuilder {
        ListChildTeamsBuilder::new(self, team_slug.into())
    }

    /// Creates a new `TeamRepoHandler` for the specified team,
    /// that allows you to manage this team's repositories.
    pub fn repos(&self, team_slug: impl Into<String>) -> TeamRepoHandler {
        TeamRepoHandler::new(self.crab, self.owner.clone(), team_slug.into())
    }
}
