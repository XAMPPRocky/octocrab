//! GitHub Actions
use snafu::ResultExt;

use crate::{params, Octocrab};

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
    ///     .add_selected_repo_to_org_secret("org", "secret_name", 1234)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_selected_repo_to_org_secret(
        &self,
        org: impl AsRef<str>,
        secret_name: impl AsRef<str>,
        repository_id: u64,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",
            org = org.as_ref(),
            secret_name = secret_name.as_ref(),
            repository_id = repository_id,
        );

        crate::map_github_error(self.crab._put(&route, None::<&()>).await?)
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
    ///     .remove_selected_repo_from_org_secret("org", "secret_name", 1234)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_selected_repo_from_org_secret(
        &self,
        org: impl AsRef<str>,
        secret_name: impl AsRef<str>,
        repository_id: u64,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",
            org = org.as_ref(),
            secret_name = secret_name.as_ref(),
            repository_id = repository_id,
        );

        crate::map_github_error(self.crab._delete(&route, None::<&()>).await?)
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
    ///     .cancel_workflow_run("owner", "repo", 1234)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_workflow_run(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: u64,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/cancel",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        crate::map_github_error(self.crab._post(&route, None::<&()>).await?)
            .await
            .map(drop)
    }

    async fn follow_location_to_data(
        &self,
        response: reqwest::Response,
    ) -> crate::Result<bytes::Bytes> {
        let location = response
            .headers()
            .get(reqwest::header::LOCATION)
            .expect("No Location header found in download_workflow_run_logs")
            .to_str()
            .expect("Location URL not valid str");

        self.crab
            ._get(location, None::<&()>)
            .await?
            .bytes()
            .await
            .context(crate::error::Http)
    }

    /// Downloads and returns the raw data representing a zip of the logs from
    /// the workflow run specified by `run_id`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .download_workflow_run_logs("owner", "repo", 1234)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_workflow_run_logs(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: u64,
    ) -> crate::Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/logs",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        self.follow_location_to_data(self.crab._get(&route, None::<&()>).await?)
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
    ///     .download_artifact("owner", "repo", 1234, ArchiveFormat::Zip)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_artifact(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        artifact_id: u64,
        archive_format: params::actions::ArchiveFormat,
    ) -> crate::Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            artifact_id = artifact_id,
            archive_format = archive_format,
        );

        self.follow_location_to_data(self.crab._get(&route, None::<&()>).await?)
            .await
    }

    /// Deletes all logs for a workflow run. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .delete_workflow_run_logs("owner", "repo", 1234)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_workflow_run_logs(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: u64,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/logs",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        crate::map_github_error(self.crab._delete(&route, None::<&()>).await?)
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
            ._post(self.handler.crab.absolute_url("/markdown")?, Some(&self))
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
