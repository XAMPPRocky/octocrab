use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListTagsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'r> ListTagsBuilder<'octo, 'r> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::Tag>> {
        let url = format!(
            "repos/{owner}/{repo}/tags",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}
