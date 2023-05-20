use super::*;
use chrono::{DateTime, Utc};

/// Handles query data for the `GET /gists` endpoint.
#[derive(Debug, serde::Serialize)]
pub struct ListAllGistsBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    /// Only show gists that were created after this UTC timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    /// The maximum number of results in each page retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    /// The page number to fetch. This starts at (and defaults to) 1
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListAllGistsBuilder<'octo> {
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// Only show gists that were created after this UTC timestamp.
    pub fn since(mut self, created_after: impl Into<DateTime<Utc>>) -> Self {
        self.since = Some(created_after.into());
        self
    }

    /// The maximum number of results in each page retrieved.
    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    /// The page number to fetch. This starts at (and defaults to) 1
    pub fn page(mut self, number: u32) -> Self {
        self.page = Some(number);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::gists::Gist>> {
        self.crab.get("/gists", Some(&self)).await
    }
}
