use serde::Serialize;

use crate::api::orgs::OrgHandler;
use crate::models::orgs::{OrgIssueField, OrgIssueType};
use crate::Result;

/// Handler for organization issue fields.
pub struct OrgIssueFieldsHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgIssueFieldsHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List issue fields for an organization.
    pub async fn list(&self) -> Result<Vec<OrgIssueField>> {
        let route = format!("/orgs/{}/issue-fields", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create an issue field for an organization.
    pub async fn create(&self, body: &impl Serialize) -> Result<OrgIssueField> {
        let route = format!("/orgs/{}/issue-fields", self.handler.owner);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Update an issue field for an organization.
    pub async fn update(
        &self,
        issue_field_id: u64,
        body: &impl Serialize,
    ) -> Result<OrgIssueField> {
        let route = format!(
            "/orgs/{}/issue-fields/{}",
            self.handler.owner, issue_field_id
        );
        self.handler.crab.patch(route, Some(body)).await
    }

    /// Delete an issue field for an organization.
    pub async fn delete(&self, issue_field_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/issue-fields/{}",
            self.handler.owner, issue_field_id
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

/// Handler for organization issue types.
pub struct OrgIssueTypesHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgIssueTypesHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List issue types for an organization.
    pub async fn list(&self) -> Result<Vec<OrgIssueType>> {
        let route = format!("/orgs/{}/issue-types", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create an issue type for an organization.
    pub async fn create(&self, body: &impl Serialize) -> Result<OrgIssueType> {
        let route = format!("/orgs/{}/issue-types", self.handler.owner);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Update an issue type for an organization.
    pub async fn update(&self, issue_type_id: u64, body: &impl Serialize) -> Result<OrgIssueType> {
        let route = format!("/orgs/{}/issue-types/{}", self.handler.owner, issue_type_id);
        self.handler.crab.put(route, Some(body)).await
    }

    /// Delete an issue type for an organization.
    pub async fn delete(&self, issue_type_id: u64) -> Result<()> {
        let route = format!("/orgs/{}/issue-types/{}", self.handler.owner, issue_type_id);
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}
