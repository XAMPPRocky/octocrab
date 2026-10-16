use serde::Serialize;

use crate::api::orgs::OrgHandler;
use crate::Result;

/// Handler for organization attestations API.
pub struct OrgAttestationsHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgAttestationsHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List attestations by bulk subject digests.
    pub async fn bulk_list<S: AsRef<str>>(&self, digests: &[S]) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/attestations/bulk-list", self.handler.owner);
        let list: Vec<&str> = digests.iter().map(AsRef::as_ref).collect();
        let body = serde_json::json!({ "subject_digests": list });
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Delete attestations in bulk.
    pub async fn delete_request(&self, body: &impl Serialize) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/attestations/delete-request", self.handler.owner);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Delete attestations by subject digest.
    pub async fn delete_by_digest(&self, subject_digest: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/attestations/digest/{}",
            self.handler.owner,
            subject_digest.as_ref()
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List attestation repositories.
    pub async fn list_repositories(&self) -> Result<serde_json::Value> {
        let route = format!("/orgs/{}/attestations/repositories", self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Delete attestations by ID.
    pub async fn delete_by_id(&self, attestation_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/attestations/{}",
            self.handler.owner, attestation_id
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List attestations for a subject digest.
    pub async fn list(&self, subject_digest: impl AsRef<str>) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/attestations/{}",
            self.handler.owner,
            subject_digest.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}
