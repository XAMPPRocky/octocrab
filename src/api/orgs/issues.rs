use super::OrgHandler;
use crate::models::issues::Issue;
use crate::{params, Page, Result};
use chrono::{DateTime, Utc};

/// Builder for listing organization issues assigned to the authenticated user.
///
/// See: [GitHub API Documentation](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-organization-issues-assigned-to-the-authenticated-user)
#[derive(serde::Serialize)]
pub struct ListOrgIssuesBuilder<'octo, 'r, 'd> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<params::issues::IssueFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "comma_separated")]
    labels: Option<&'d [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<params::issues::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r, 'd> ListOrgIssuesBuilder<'octo, 'r, 'd> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            filter: None,
            state: None,
            labels: None,
            sort: None,
            direction: None,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter issues by type.
    pub fn filter(mut self, filter: params::issues::IssueFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Filter issues by state (`open`, `closed`, `all`).
    pub fn state(mut self, state: params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter issues by labels.
    pub fn labels(mut self, labels: &'d (impl AsRef<[String]> + ?Sized)) -> Self {
        self.labels = Some(labels.as_ref());
        self
    }

    /// What to sort results by (`created`, `updated`, `comments`).
    pub fn sort(mut self, sort: impl Into<params::issues::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort (`asc`, `desc`).
    pub fn direction(mut self, direction: impl Into<params::Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Only show issues updated at or after this time.
    pub fn since(mut self, since: impl Into<DateTime<Utc>>) -> Self {
        self.since = Some(since.into());
        self
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

    /// Send the actual request.
    pub async fn send(self) -> Result<Page<Issue>> {
        let route = format!("/orgs/{org}/issues", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

fn comma_separated<S: serde::Serializer>(
    labels: &Option<&[String]>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    serializer.serialize_str(&labels.unwrap().join(","))
}
