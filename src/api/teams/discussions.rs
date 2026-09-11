use crate::models::{
    reactions::{Reaction, ReactionContent},
    ReactionId, TeamId,
};
use crate::{Octocrab, Page, Result};

#[derive(Clone)]
pub(crate) enum TeamDiscussionTarget {
    OrgAndSlug { org: String, team_slug: String },
    Id(TeamId),
}

/// A builder pattern struct for listing reactions for a team discussion.
///
/// Created by [`TeamHandler::list_discussion_reactions`] or [`TeamByIdHandler::list_discussion_reactions`].
#[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
#[derive(serde::Serialize)]
pub struct ListTeamDiscussionReactionsBuilder<'octo, 'r> {
    #[serde(skip)]
    crab: &'r Octocrab,
    #[serde(skip)]
    target: TeamDiscussionTarget,
    #[serde(skip)]
    discussion_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<ReactionContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip)]
    _octo: std::marker::PhantomData<&'octo ()>,
}

#[allow(deprecated)]
impl<'octo, 'r> ListTeamDiscussionReactionsBuilder<'octo, 'r> {
    pub(crate) fn new(
        crab: &'r Octocrab,
        target: TeamDiscussionTarget,
        discussion_number: u64,
    ) -> Self {
        Self {
            crab,
            target,
            discussion_number,
            content: None,
            per_page: None,
            page: None,
            _octo: std::marker::PhantomData,
        }
    }

    /// Filter reactions by type.
    pub fn content(mut self, content: ReactionContent) -> Self {
        self.content = Some(content);
        self
    }

    /// Results per page (max 100).
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
    pub async fn send(self) -> Result<Page<Reaction>> {
        let route = match &self.target {
            TeamDiscussionTarget::OrgAndSlug { org, team_slug } => {
                format!(
                    "/orgs/{org}/teams/{team_slug}/discussions/{}/reactions",
                    self.discussion_number
                )
            }
            TeamDiscussionTarget::Id(team_id) => {
                format!(
                    "/teams/{team_id}/discussions/{}/reactions",
                    self.discussion_number
                )
            }
        };
        self.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for listing reactions for a team discussion comment.
///
/// Created by [`TeamHandler::list_discussion_comment_reactions`] or [`TeamByIdHandler::list_discussion_comment_reactions`].
#[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
#[derive(serde::Serialize)]
pub struct ListTeamDiscussionCommentReactionsBuilder<'octo, 'r> {
    #[serde(skip)]
    crab: &'r Octocrab,
    #[serde(skip)]
    target: TeamDiscussionTarget,
    #[serde(skip)]
    discussion_number: u64,
    #[serde(skip)]
    comment_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<ReactionContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip)]
    _octo: std::marker::PhantomData<&'octo ()>,
}

#[allow(deprecated)]
impl<'octo, 'r> ListTeamDiscussionCommentReactionsBuilder<'octo, 'r> {
    pub(crate) fn new(
        crab: &'r Octocrab,
        target: TeamDiscussionTarget,
        discussion_number: u64,
        comment_number: u64,
    ) -> Self {
        Self {
            crab,
            target,
            discussion_number,
            comment_number,
            content: None,
            per_page: None,
            page: None,
            _octo: std::marker::PhantomData,
        }
    }

    /// Filter reactions by type.
    pub fn content(mut self, content: ReactionContent) -> Self {
        self.content = Some(content);
        self
    }

    /// Results per page (max 100).
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
    pub async fn send(self) -> Result<Page<Reaction>> {
        let route = match &self.target {
            TeamDiscussionTarget::OrgAndSlug { org, team_slug } => {
                format!(
                    "/orgs/{org}/teams/{team_slug}/discussions/{}/comments/{}/reactions",
                    self.discussion_number, self.comment_number
                )
            }
            TeamDiscussionTarget::Id(team_id) => {
                format!(
                    "/teams/{team_id}/discussions/{}/comments/{}/reactions",
                    self.discussion_number, self.comment_number
                )
            }
        };
        self.crab.get(route, Some(&self)).await
    }
}

/// Handler for GitHub's team API identified by Team ID.
///
/// Created with [`Octocrab::teams_by_id`].
#[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
pub struct TeamByIdHandler<'octo> {
    crab: &'octo Octocrab,
    team_id: TeamId,
}

#[allow(deprecated)]
impl<'octo> TeamByIdHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, team_id: TeamId) -> Self {
        Self { crab, team_id }
    }

    /// Lists reactions for a team discussion.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28)
    #[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
    pub fn list_discussion_reactions(
        &self,
        discussion_number: u64,
    ) -> ListTeamDiscussionReactionsBuilder<'octo, '_> {
        ListTeamDiscussionReactionsBuilder::new(
            self.crab,
            TeamDiscussionTarget::Id(self.team_id),
            discussion_number,
        )
    }

    /// Creates a reaction for a team discussion.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28)
    #[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
    pub async fn create_discussion_reaction(
        &self,
        discussion_number: u64,
        content: ReactionContent,
    ) -> Result<Reaction> {
        let route = format!(
            "/teams/{}/discussions/{discussion_number}/reactions",
            self.team_id
        );
        self.crab
            .post(route, Some(&serde_json::json!({ "content": content })))
            .await
    }

    /// Deletes a reaction for a team discussion.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28)
    #[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
    pub async fn delete_discussion_reaction(
        &self,
        discussion_number: u64,
        reaction_id: impl Into<ReactionId>,
    ) -> Result<()> {
        let reaction_id = reaction_id.into();
        let route = format!(
            "/teams/{}/discussions/{discussion_number}/reactions/{reaction_id}",
            self.team_id
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Lists reactions for a team discussion comment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28)
    #[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
    pub fn list_discussion_comment_reactions(
        &self,
        discussion_number: u64,
        comment_number: u64,
    ) -> ListTeamDiscussionCommentReactionsBuilder<'octo, '_> {
        ListTeamDiscussionCommentReactionsBuilder::new(
            self.crab,
            TeamDiscussionTarget::Id(self.team_id),
            discussion_number,
            comment_number,
        )
    }

    /// Creates a reaction for a team discussion comment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28)
    #[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
    pub async fn create_discussion_comment_reaction(
        &self,
        discussion_number: u64,
        comment_number: u64,
        content: ReactionContent,
    ) -> Result<Reaction> {
        let route = format!(
            "/teams/{}/discussions/{discussion_number}/comments/{comment_number}/reactions",
            self.team_id
        );
        self.crab
            .post(route, Some(&serde_json::json!({ "content": content })))
            .await
    }

    /// Deletes a reaction for a team discussion comment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/reactions/reactions?apiVersion=2022-11-28)
    #[deprecated(note = "Team Discussions have been deprecated and sunset by GitHub.")]
    pub async fn delete_discussion_comment_reaction(
        &self,
        discussion_number: u64,
        comment_number: u64,
        reaction_id: impl Into<ReactionId>,
    ) -> Result<()> {
        let reaction_id = reaction_id.into();
        let route = format!(
            "/teams/{}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id}",
            self.team_id
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }
}
