use super::*;
use crate::{models, Page, Result};

#[derive(serde::Serialize)]
pub struct ListChildTeamsBuilder<'octo, 'a> {
    #[serde(skip)]
    handler: &'a TeamHandler<'octo>,
    #[serde(skip)]
    slug: String,
    per_page: Option<u8>,
    page: Option<u32>,
}

impl<'octo, 'a> ListChildTeamsBuilder<'octo, 'a> {
    pub(crate) fn new(handler: &'a TeamHandler<'octo>, slug: String) -> Self {
        Self {
            handler,
            slug,
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
        let url = format!(
            "/orgs/{org}/teams/{team}/teams",
            org = self.handler.owner,
            team = self.slug,
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}
