use serde::Serialize;

use crate::api::orgs::OrgHandler;
use crate::models::orgs::OrgArtifactDeploymentRecordsResponse;
use crate::Result;

/// Handler for organization artifact metadata API.
pub struct OrgArtifactMetadataHandler<'octo, 'b> {
    handler: &'b OrgHandler<'octo>,
}

impl<'octo, 'b> OrgArtifactMetadataHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Create an artifact deployment record.
    pub async fn create_deployment_record(
        &self,
        body: &impl Serialize,
    ) -> Result<OrgArtifactDeploymentRecordsResponse> {
        let route = format!(
            "/orgs/{}/artifacts/metadata/deployment-record",
            self.handler.owner
        );
        self.handler.crab.post(route, Some(body)).await
    }

    /// Set cluster deployment records.
    pub async fn set_cluster_deployment_records(
        &self,
        cluster: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/artifacts/metadata/deployment-record/cluster/{}",
            self.handler.owner,
            cluster.as_ref()
        );
        let res = self.handler.crab._post(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Create a cluster deployment records job.
    pub async fn create_cluster_deployment_records_job(
        &self,
        cluster: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/artifacts/metadata/deployment-record/cluster/{}/jobs",
            self.handler.owner,
            cluster.as_ref()
        );
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get cluster deployment records job status.
    pub async fn get_cluster_deployment_records_job(
        &self,
        cluster: impl AsRef<str>,
        job_id: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/artifacts/metadata/deployment-record/cluster/{}/jobs/{}",
            self.handler.owner,
            cluster.as_ref(),
            job_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create artifact metadata storage record.
    pub async fn create_storage_record(&self, body: &impl Serialize) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/artifacts/metadata/storage-record",
            self.handler.owner
        );
        self.handler.crab.post(route, Some(body)).await
    }

    /// List artifact deployment records.
    pub async fn list_deployment_records(
        &self,
        subject_digest: impl AsRef<str>,
    ) -> Result<OrgArtifactDeploymentRecordsResponse> {
        let route = format!(
            "/orgs/{}/artifacts/{}/metadata/deployment-records",
            self.handler.owner,
            subject_digest.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// List artifact storage records.
    pub async fn list_storage_records(
        &self,
        subject_digest: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/artifacts/{}/metadata/storage-records",
            self.handler.owner,
            subject_digest.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}
