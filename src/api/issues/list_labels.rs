use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListLabelsForIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    number: u64,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'r> ListLabelsForIssueBuilder<'octo, 'r> {
    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::Label>> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{number}/labels",
            owner = self.handler.owner,
            repo = self.handler.repo,
            number = self.number,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListLabelsForRepoBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'r> ListLabelsForRepoBuilder<'octo, 'r> {
    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::Label>> {
        let route = format!(
            "repos/{owner}/{repo}/labels",
            owner = self.handler.owner,
            repo = self.handler.repo,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}
