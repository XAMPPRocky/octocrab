use crate::models::orgs::Organization;
use crate::{Octocrab, Page};

/// A builder pattern struct for listing all organizations.
///
/// Created by [`Octocrab::all_organizations`].
#[derive(serde::Serialize)]
pub struct ListAllOrganizationsBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
}

impl<'octo> ListAllOrganizationsBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            since: None,
            per_page: None,
        }
    }

    /// An organization ID to start listing organizations from.
    pub fn since(mut self, since: impl Into<u64>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Sends the actual request.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/orgs?apiVersion=2022-11-28#list-organizations)
    pub async fn send(self) -> crate::Result<Page<Organization>> {
        let route = "/organizations";
        self.crab.get(route, Some(&self)).await
    }
}
