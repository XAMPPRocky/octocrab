use super::*;
use crate::{models, Page, Result};

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListChildTeamsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamHandler<'octo>,
    #[serde(skip)]
    slug: String,
    /// Results per page.
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'r> ListChildTeamsBuilder<'octo, 'r> {
    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::teams::RequestedTeam>> {
        let url = format!(
            "orgs/{org}/teams/{team}/teams",
            org = self.handler.owner,
            team = self.slug,
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}
