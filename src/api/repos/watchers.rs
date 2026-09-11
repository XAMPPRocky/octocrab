//! GitHub Repository Watchers API

use serde::Serialize;

use crate::models::Author;
use crate::{Octocrab, Page, Result};

#[derive(Serialize)]
pub struct ListWatchersBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListWatchersBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, route: impl Into<String>) -> Self {
        Self {
            crab,
            route: route.into(),
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
    pub async fn send(self) -> Result<Page<Author>> {
        self.crab.get(&self.route, Some(&self)).await
    }
}
