use super::*;
use crate::{models, Page, Result};

#[derive(serde::Serialize)]
pub struct ListTeamsBuilder<'octo, 'a> {
    #[serde(skip)]
    handler: &'a TeamHandler<'octo>,
    per_page: Option<u8>,
    page: Option<u32>,
}

impl<'octo, 'a> ListTeamsBuilder<'octo, 'a> {
    pub(crate) fn new(handler: &'a TeamHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> Result<Page<models::RequestedTeam>> {
        let url = format!("/orgs/{owner}/teams", owner = self.handler.owner);
        self.handler.crab.get(url, Some(&self)).await
    }
}
