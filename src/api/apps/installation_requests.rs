use super::*;
use crate::models::apps::InstallationRequest;
use crate::Page;

/// A builder pattern struct for listing installation requests.
///
/// Created by [`AppsRequestHandler::installation_requests`].
#[derive(serde::Serialize)]
pub struct InstallationRequestsBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> InstallationRequestsBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
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
    pub async fn send(self) -> crate::Result<Page<InstallationRequest>> {
        let route = "/app/installation-requests";
        self.crab.get(route, Some(&self)).await
    }
}
