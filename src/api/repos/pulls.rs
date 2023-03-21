use super::*;

#[derive(serde::Serialize)]
pub struct ListPullsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListPullsBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>, sha: String) -> Self {
        Self {
            handler,
            sha,
            per_page: None,
            page: None,
        }
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::pulls::PullRequest>> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{sha}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo,
            sha = self.sha,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
