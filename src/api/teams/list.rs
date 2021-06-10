use super::*;
use crate::{models, Page, Result};

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListTeamsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamHandler<'octo>,
    /// Results per page.
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'r> ListTeamsBuilder<'octo, 'r> {
    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::teams::RequestedTeam>> {
        let url = format!("orgs/{owner}/teams", owner = self.handler.owner);
        self.handler.crab.get(url, Some(&self)).await
    }
}
