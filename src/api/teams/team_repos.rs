use crate::params;
use crate::{Octocrab, Result};
use reqwest::StatusCode;

pub struct TeamRepoHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
    team: String,
}

pub enum ManagesRepo {
    Yes,
    No,
    Err,
}

impl<'octo> TeamRepoHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String, team: String) -> Self {
        Self { crab, org, team }
    }

    pub async fn manages(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
    ) -> Result<ManagesRepo> {
        let url = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        let res = self.crab._get(&url, None::<&()>).await?;
        Ok(match res.status() {
            StatusCode::NO_CONTENT => ManagesRepo::Yes,
            StatusCode::NOT_FOUND => ManagesRepo::No,
            _ => ManagesRepo::Err,
        })
    }

    pub async fn add_or_update(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
        permission: Option<params::teams::Permission>,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        self.crab.put(url, permission.as_ref()).await
    }

    pub async fn remove(
        &self,
        repo_owner: impl Into<String>,
        repo_name: impl Into<String>,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{org}/teams/{team}/repos/{owner}/{repo}",
            org = self.org,
            team = self.team,
            owner = repo_owner.into(),
            repo = repo_name.into(),
        );
        self.crab.delete(url, None::<&()>).await
    }
}
