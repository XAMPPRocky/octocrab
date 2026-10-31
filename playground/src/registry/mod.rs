pub mod entity;
pub mod gists;
pub mod issues;
pub mod orgs;
pub mod pulls;
pub mod repos;
pub mod search;
pub mod users;

pub use entity::{EntityDefinition, ParentContext};
use gists::{GistCommentsEntity, GistEntity};
use issues::{IssueCommentsEntity, IssueEntity};
use orgs::{OrgEntity, OrgMembersEntity, OrgReposEntity, OrgTeamsEntity};
use pulls::{PullCommentsEntity, PullCommitsEntity, PullFilesEntity, PullRequestEntity};
use repos::{
    RepoBranchesEntity, RepoCommitsEntity, RepoContributorsEntity, RepoEntity, RepoIssuesEntity,
    RepoLabelsEntity, RepoLanguagesEntity, RepoMilestonesEntity, RepoPullsEntity,
    RepoReleasesEntity, RepoTagsEntity, RepoWorkflowsEntity,
};
use search::{SearchIssuesEntity, SearchRepositoriesEntity};
use users::{CurrentUserEntity, UserEntity, UserFollowersEntity, UserGistsEntity, UserReposEntity};

pub fn all_entities() -> Vec<Box<dyn EntityDefinition>> {
    vec![
        // Organizations
        Box::new(OrgEntity),
        Box::new(OrgReposEntity),
        Box::new(OrgMembersEntity),
        Box::new(OrgTeamsEntity),
        // Repositories
        Box::new(RepoEntity),
        Box::new(RepoIssuesEntity),
        Box::new(RepoPullsEntity),
        Box::new(RepoCommitsEntity),
        Box::new(RepoReleasesEntity),
        Box::new(RepoBranchesEntity),
        Box::new(RepoWorkflowsEntity),
        Box::new(RepoTagsEntity),
        Box::new(RepoContributorsEntity),
        Box::new(RepoLanguagesEntity),
        Box::new(RepoLabelsEntity),
        Box::new(RepoMilestonesEntity),
        // Users
        Box::new(UserEntity),
        Box::new(CurrentUserEntity),
        Box::new(UserReposEntity),
        Box::new(UserFollowersEntity),
        Box::new(UserGistsEntity),
        // Gists
        Box::new(GistEntity),
        Box::new(GistCommentsEntity),
        // Issues
        Box::new(IssueEntity),
        Box::new(IssueCommentsEntity),
        // Pull Requests
        Box::new(PullRequestEntity),
        Box::new(PullCommitsEntity),
        Box::new(PullFilesEntity),
        Box::new(PullCommentsEntity),
        // Search
        Box::new(SearchRepositoriesEntity),
        Box::new(SearchIssuesEntity),
    ]
}

pub fn get_entity(id: &str) -> Option<Box<dyn EntityDefinition>> {
    all_entities().into_iter().find(|e| e.id() == id)
}

pub fn all_root_entities() -> Vec<Box<dyn EntityDefinition>> {
    all_entities().into_iter().filter(|e| e.is_root()).collect()
}

pub fn allowed_child_entities(parent_id: &str) -> Vec<Box<dyn EntityDefinition>> {
    let parent = match get_entity(parent_id) {
        Some(p) => p,
        None => return Vec::new(),
    };

    let allowed_ids = parent.allowed_children();
    all_entities()
        .into_iter()
        .filter(|e| allowed_ids.contains(&e.id()))
        .collect()
}
