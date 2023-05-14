//! GitHub Actions
use snafu::ResultExt;

use crate::error::{HttpSnafu, HyperSnafu};
use crate::etag::{EntityTag, Etagged};
use crate::models::{
    workflows::WorkflowDispatch, workflows::WorkflowListArtifact, ArtifactId, RepositoryId, RunId,
};
use crate::{params, FromResponse, Octocrab, Page};
use http::request::Builder;
use http::{header::HeaderMap, Method, StatusCode, Uri};
use hyper::body;

pub struct ListWorkflowRunArtifacts<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
    run_id: RunId,
    per_page: Option<u8>,
    page: Option<u32>,
    etag: Option<EntityTag>,
}

impl<'octo> ListWorkflowRunArtifacts<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String, run_id: RunId) -> Self {
        Self {
            crab,
            owner,
            repo,
            run_id,
            per_page: None,
            page: None,
            etag: None,
        }
    }

    /// Etag for this request.
    pub fn etag(mut self, etag: Option<EntityTag>) -> Self {
        self.etag = etag;
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

    pub async fn send(self) -> crate::Result<Etagged<Page<WorkflowListArtifact>>> {
        let path = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts",
            owner = self.owner,
            repo = self.repo,
            run_id = self.run_id
        );
        let uri = Uri::builder()
            .path_and_query(path)
            .build()
            .context(HttpSnafu)?;
        let mut headers = HeaderMap::new();
        if let Some(etag) = self.etag {
            EntityTag::insert_if_none_match_header(&mut headers, etag)?;
        }

        let request = self
            .crab
            .build_request(Builder::new().method(Method::GET).uri(uri), None::<&()>)?;
        let response = self.crab.execute(request).await?;
        let etag = EntityTag::extract_from_response(&response);
        if response.status() == StatusCode::NOT_MODIFIED {
            Ok(Etagged { etag, value: None })
        } else {
            <Page<WorkflowListArtifact>>::from_response(crate::map_github_error(response).await?)
                .await
                .map(|page| Etagged {
                    etag,
                    value: Some(page),
                })
        }
    }
}

pub struct WorkflowDispatchBuilder<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
    workflow_id: String,
    data: WorkflowDispatch,
}

impl<'octo> WorkflowDispatchBuilder<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        owner: String,
        repo: String,
        workflow_id: String,
        r#ref: String,
    ) -> Self {
        let mut this = Self {
            crab,
            owner,
            repo,
            workflow_id,
            data: Default::default(),
        };
        this.data.r#ref = r#ref;
        this
    }

    /// Input keys and values configured in the workflow file. The maximum number of properties is 10.
    /// Any default properties configured in the workflow file will be used when inputs are omitted.
    ///
    /// # Panics
    /// If `inputs` is not `Value::Object`.
    pub fn inputs(mut self, inputs: serde_json::Value) -> Self {
        assert!(inputs.is_object(), "Inputs should be a JSON object");
        self.data.inputs = inputs;
        self
    }

    pub async fn send(self) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches",
            owner = self.owner,
            repo = self.repo,
            workflow_id = self.workflow_id
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        // this entry point doesn't actually return anything sensible
        self.crab._post(uri, Some(&self.data)).await?;

        Ok(())
    }
}

/// Handler for GitHub's actions API.
///
/// Created with [`Octocrab::actions`].
pub struct ActionsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ActionsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Adds a repository to an organization secret when the visibility for
    /// repository access is set to selected. The visibility is set when you
    /// create or update an organization secret. You must authenticate using an
    /// access token with the admin:org scope to use this endpoint. GitHub Apps
    /// must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .add_selected_repo_to_org_secret("org", "secret_name", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_selected_repo_to_org_secret(
        &self,
        org: impl AsRef<str>,
        secret_name: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",
            org = org.as_ref(),
            secret_name = secret_name.as_ref(),
            repository_id = repository_id,
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._put(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Removes a repository from an organization secret when the visibility for
    /// repository access is set to selected. The visibility is set when you
    /// create or update an organization secret. You must authenticate using an
    /// access token with the admin:org scope to use this endpoint. GitHub Apps
    /// must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .remove_selected_repo_from_org_secret("org", "secret_name", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_selected_repo_from_org_secret(
        &self,
        org: impl AsRef<str>,
        secret_name: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",
            org = org.as_ref(),
            secret_name = secret_name.as_ref(),
            repository_id = repository_id,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Cancels a workflow run using its id. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .cancel_workflow_run("owner", "repo", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_workflow_run(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: RunId,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/cancel",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._post(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    async fn follow_location_to_data(
        &self,
        response: http::Response<hyper::Body>,
    ) -> crate::Result<bytes::Bytes> {
        let data_response = self.crab.follow_location_to_data(response).await?;

        let body = data_response.into_body();

        body::to_bytes(body).await.context(HyperSnafu)
    }

    /// Downloads and returns the raw data representing a zip of the logs from
    /// the workflow run specified by `run_id`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .download_workflow_run_logs("owner", "repo", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_workflow_run_logs(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: RunId,
    ) -> crate::Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/logs",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        self.follow_location_to_data(self.crab._get(uri).await?)
            .await
    }

    /// Downloads and returns the raw data representing an artifact from a
    /// repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::actions::ArchiveFormat;
    ///
    /// octocrab::instance()
    ///     .actions()
    ///     .download_artifact("owner", "repo", 1234u64.into(), ArchiveFormat::Zip)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_artifact(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        artifact_id: ArtifactId,
        archive_format: params::actions::ArchiveFormat,
    ) -> crate::Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            artifact_id = artifact_id,
            archive_format = archive_format,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        self.follow_location_to_data(self.crab._get(uri).await?)
            .await
    }

    /// Deletes all logs for a workflow run. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .delete_workflow_run_logs("owner", "repo", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_workflow_run_logs(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: RunId,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/logs",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Get an organization's public key, which you need to encrypt secrets.
    /// You need to encrypt a secret before you can create or update secrets.
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the secrets organization
    /// permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.actions().get_org_public_key("org").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_org_public_key(
        &self,
        org: impl AsRef<str>,
    ) -> crate::Result<crate::models::PublicKey> {
        let route = format!("/orgs/{org}/actions/secrets/public-key", org = org.as_ref());

        self.crab.get(route, None::<&()>).await
    }

    /// Lists artifacts for a workflow run. Anyone with read access to the
    /// repository can use this endpoint. If the repository is private you
    /// must use an access token with the `repo` scope. GitHub Apps must have
    /// the `actions:read` permission to use this endpoint.
    pub fn list_workflow_run_artifacts(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        run_id: RunId,
    ) -> ListWorkflowRunArtifacts<'_> {
        ListWorkflowRunArtifacts::new(self.crab, owner.into(), repo.into(), run_id)
    }

    /// Dispatch a workflow run. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.actions()
    ///    .create_workflow_dispatch("org", "repo", "workflow.yaml", "ref")
    ///    // optional
    ///    .inputs(serde_json::json!({"my-key": "my-value"}))
    ///    .send()
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub fn create_workflow_dispatch(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        workflow_id: impl Into<String>,
        r#ref: impl Into<String>,
    ) -> WorkflowDispatchBuilder<'_> {
        WorkflowDispatchBuilder::new(
            self.crab,
            owner.into(),
            repo.into(),
            workflow_id.into(),
            r#ref.into(),
        )
    }
}

/*
#[derive(serde::Serialize)]
pub struct RenderMarkdownBuilder<'octo, 'r, 'text> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    text: &'text str,
    mode: Option<crate::params::markdown::Mode>,
    context: Option<String>,
}

impl<'octo, 'r, 'text> RenderMarkdownBuilder<'octo, 'r, 'text> {
    pub(crate) fn new(handler: &'r ActionsHandler<'octo>, text: &'text str) -> Self {
        Self {
            handler,
            text,
            mode: None,
            context: None,
        }
    }

    /// The repository context to use when creating references in `Mode::Gfm`.
    /// Omit this parameter when using markdown mode.
    pub fn context<A: Into<String>>(mut self, context: impl Into<Option<A>>) -> Self {
        self.context = context.into().map(A::into);
        self
    }

    /// The rendering mode.
    pub fn mode(mut self, mode: impl Into<Option<crate::params::markdown::Mode>>) -> Self {
        self.mode = mode.into();
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<String> {
        self.handler
            .crab
            ._post(self.handler.crab.absolute_url("markdown")?, Some(&self))
            .await?
            .text()
            .await
            .context(crate::error::Http)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn serialize() {
        let octocrab = crate::instance();
        let handler = octocrab.markdown();
        let render = handler
            .render("**Markdown**")
            .mode(crate::params::markdown::Mode::Gfm)
            .context("owner/repo");


        assert_eq!(
            serde_json::to_value(render).unwrap(),
            serde_json::json!({
                "text": "**Markdown**",
                "mode": "gfm",
                "context": "owner/repo",
            })
        )
    }
}
*/
