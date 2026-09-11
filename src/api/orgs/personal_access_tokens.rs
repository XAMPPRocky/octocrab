use super::OrgHandler;
use crate::models::orgs::personal_access_tokens::{
    OrgPersonalAccessToken, OrgPersonalAccessTokenRequest, PatAction, PatReviewDecision,
};
use crate::models::{PatId, PatRequestId, Repository};
use crate::{Page, Result};

/// A client to GitHub's organization personal access tokens API.
///
/// Created with [`OrgHandler::personal_access_tokens`].
pub struct OrgPersonalAccessTokensHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgPersonalAccessTokensHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists fine-grained personal access tokens with access to organization resources.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-tokens?apiVersion=2022-11-28#list-fine-grained-personal-access-tokens-with-access-to-organization-resources)
    pub fn list(&self) -> ListOrgPersonalAccessTokensBuilder<'octo, 'r> {
        ListOrgPersonalAccessTokensBuilder::new(self.handler)
    }

    /// Gets a fine-grained personal access token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-tokens?apiVersion=2022-11-28#get-a-fine-grained-personal-access-token)
    pub async fn get(&self, pat_id: impl Into<PatId>) -> Result<OrgPersonalAccessToken> {
        let pat_id = pat_id.into();
        let route = format!(
            "/orgs/{org}/personal-access-tokens/{pat_id}",
            org = self.handler.owner
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Updates the access to organization resources for a fine-grained personal access token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-tokens?apiVersion=2022-11-28#update-the-access-to-organization-resources-for-a-fine-grained-personal-access-token)
    pub async fn update_access(&self, pat_id: impl Into<PatId>, action: PatAction) -> Result<()> {
        let pat_id = pat_id.into();
        let route = format!(
            "/orgs/{org}/personal-access-tokens/{pat_id}",
            org = self.handler.owner
        );
        let body = serde_json::json!({ "action": action });
        let response = self.handler.crab._post(route, Some(&body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists repositories a fine-grained personal access token has access to.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-tokens?apiVersion=2022-11-28#list-repositories-a-fine-grained-personal-access-token-has-access-to)
    pub fn list_repositories(
        &self,
        pat_id: impl Into<PatId>,
    ) -> ListOrgPatRepositoriesBuilder<'octo, 'r> {
        ListOrgPatRepositoriesBuilder::new(self.handler, pat_id.into())
    }

    /// Lists personal access token requests to access organization resources.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-token-requests?apiVersion=2022-11-28#list-personal-access-token-requests-to-access-organization-resources)
    pub fn list_requests(&self) -> ListOrgPatRequestsBuilder<'octo, 'r> {
        ListOrgPatRequestsBuilder::new(self.handler)
    }

    /// Reviews requests to access organization resources in batch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-token-requests?apiVersion=2022-11-28#review-requests-to-access-organization-resources)
    pub async fn review_requests(
        &self,
        pat_request_ids: impl IntoIterator<Item = impl Into<PatRequestId>>,
        decision: PatReviewDecision,
        reason: Option<String>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/personal-access-token-requests",
            org = self.handler.owner
        );
        let ids: Vec<PatRequestId> = pat_request_ids.into_iter().map(Into::into).collect();
        let mut body = serde_json::json!({
            "pat_request_ids": ids,
            "decision": decision
        });
        if let Some(reason) = reason {
            body["reason"] = serde_json::Value::String(reason);
        }
        let response = self.handler.crab._post(route, Some(&body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Reviews a single request to access organization resources.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-token-requests?apiVersion=2022-11-28#review-a-request-to-access-organization-resources)
    pub async fn review_request(
        &self,
        pat_request_id: impl Into<PatRequestId>,
        decision: PatReviewDecision,
        reason: Option<String>,
    ) -> Result<()> {
        let pat_request_id = pat_request_id.into();
        let route = format!(
            "/orgs/{org}/personal-access-token-requests/{pat_request_id}",
            org = self.handler.owner
        );
        let mut body = serde_json::json!({
            "decision": decision
        });
        if let Some(reason) = reason {
            body["reason"] = serde_json::Value::String(reason);
        }
        let response = self.handler.crab._post(route, Some(&body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists repositories requested for a personal access token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/personal-access-token-requests?apiVersion=2022-11-28#list-repositories-requested-for-a-personal-access-token)
    pub fn list_request_repositories(
        &self,
        pat_request_id: impl Into<PatRequestId>,
    ) -> ListOrgPatRequestRepositoriesBuilder<'octo, 'r> {
        ListOrgPatRequestRepositoriesBuilder::new(self.handler, pat_request_id.into())
    }
}

/// Builder for listing organization personal access tokens.
#[derive(serde::Serialize)]
pub struct ListOrgPersonalAccessTokensBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
}

impl<'octo, 'r> ListOrgPersonalAccessTokensBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            sort: None,
            direction: None,
            owner: None,
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

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn direction(mut self, direction: crate::params::Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    pub async fn send(self) -> Result<Page<OrgPersonalAccessToken>> {
        let route = format!(
            "/orgs/{org}/personal-access-tokens",
            org = self.handler.owner
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing repositories a personal access token has access to.
#[derive(serde::Serialize)]
pub struct ListOrgPatRepositoriesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip)]
    pat_id: PatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgPatRepositoriesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>, pat_id: PatId) -> Self {
        Self {
            handler,
            pat_id,
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

    pub async fn send(self) -> Result<Page<Repository>> {
        let route = format!(
            "/orgs/{org}/personal-access-tokens/{pat_id}/repositories",
            org = self.handler.owner,
            pat_id = self.pat_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing personal access token requests.
#[derive(serde::Serialize)]
pub struct ListOrgPatRequestsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
}

impl<'octo, 'r> ListOrgPatRequestsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            sort: None,
            direction: None,
            owner: None,
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

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn direction(mut self, direction: crate::params::Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    pub async fn send(self) -> Result<Page<OrgPersonalAccessTokenRequest>> {
        let route = format!(
            "/orgs/{org}/personal-access-token-requests",
            org = self.handler.owner
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing repositories requested for a personal access token.
#[derive(serde::Serialize)]
pub struct ListOrgPatRequestRepositoriesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip)]
    pat_request_id: PatRequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgPatRequestRepositoriesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>, pat_request_id: PatRequestId) -> Self {
        Self {
            handler,
            pat_request_id,
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

    pub async fn send(self) -> Result<Page<Repository>> {
        let route = format!(
            "/orgs/{org}/personal-access-token-requests/{pat_request_id}/repositories",
            org = self.handler.owner,
            pat_request_id = self.pat_request_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
