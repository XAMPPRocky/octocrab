//! GitHub Repository Pages API.
//!
//! See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28)

use super::RepoHandler;
use crate::{
    models::{
        repos::{
            PageBuild, PageBuildStatus, PagesBuildType, PagesDeployment, PagesDeploymentId,
            PagesDeploymentStatus, PagesHealthCheck, PagesSite, PagesSource, UpdatePagesSource,
        },
        PageBuildId,
    },
    Page, Result,
};

/// A client to GitHub's repository pages API.
///
/// Created with [`RepoHandler::pages`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28)
pub struct RepoPagesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoPagesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Access the builds sub-API for GitHub Pages.
    pub fn builds(&self) -> RepoPagesBuildsHandler<'octo, 'r> {
        RepoPagesBuildsHandler::new(self.handler)
    }

    /// Access the deployments sub-API for GitHub Pages.
    pub fn deployments(&self) -> RepoPagesDeploymentsHandler<'octo, 'r> {
        RepoPagesDeploymentsHandler::new(self.handler)
    }

    /// Gets information about a GitHub Pages site.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-a-github-pages-site)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let site = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<PagesSite> {
        let route = format!("/{}/pages", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a builder to configure and enable a GitHub Pages site.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#create-a-apiname-pages-site)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::repos::{PagesBuildType, PagesSource};
    ///
    /// let site = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .create()
    ///     .source_branch("main", "/")
    ///     .build_type(PagesBuildType::Legacy)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self) -> CreatePagesSiteBuilder<'octo, 'r> {
        CreatePagesSiteBuilder::new(self.handler)
    }

    /// Creates a builder to update information about a GitHub Pages site.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#update-information-about-a-apiname-pages-site)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .update()
    ///     .cname("example.com")
    ///     .https_enforced(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self) -> UpdatePagesSiteBuilder<'octo, 'r> {
        UpdatePagesSiteBuilder::new(self.handler)
    }

    /// Deletes a GitHub Pages site.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#delete-a-apiname-pages-site)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .delete()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self) -> Result<()> {
        let route = format!("/{}/pages", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Gets a DNS health check for GitHub Pages.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-a-dns-health-check-for-github-pages)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let health = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .health()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health(&self) -> Result<PagesHealthCheck> {
        let route = format!("/{}/pages/health", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a builder to list GitHub Pages builds.
    ///
    /// Convenience method for [`RepoPagesBuildsHandler::list`].
    pub fn list_builds(&self) -> ListPagesBuildsBuilder<'octo, 'r> {
        self.builds().list()
    }

    /// Gets the latest GitHub Pages build.
    ///
    /// Convenience method for [`RepoPagesBuildsHandler::latest`].
    pub async fn latest_build(&self) -> Result<PageBuild> {
        self.builds().latest().await
    }

    /// Gets a specific GitHub Pages build by ID.
    ///
    /// Convenience method for [`RepoPagesBuildsHandler::get`].
    pub async fn get_build(&self, build_id: impl Into<PageBuildId>) -> Result<PageBuild> {
        self.builds().get(build_id).await
    }

    /// Requests a new GitHub Pages build.
    ///
    /// Convenience method for [`RepoPagesBuildsHandler::request`].
    pub async fn request_build(&self) -> Result<PageBuildStatus> {
        self.builds().request().await
    }

    /// Creates a builder to create a GitHub Pages deployment.
    ///
    /// Convenience method for [`RepoPagesDeploymentsHandler::create`].
    pub fn create_deployment(
        &self,
        oidc_token: impl Into<String>,
    ) -> CreatePagesDeploymentBuilder<'octo, 'r> {
        self.deployments().create(oidc_token)
    }

    /// Gets the status of a GitHub Pages deployment.
    ///
    /// Convenience method for [`RepoPagesDeploymentsHandler::status`].
    pub async fn get_deployment_status(
        &self,
        deployment_id: impl Into<PagesDeploymentId>,
    ) -> Result<PagesDeploymentStatus> {
        self.deployments().status(deployment_id).await
    }

    /// Cancels a GitHub Pages deployment.
    ///
    /// Convenience method for [`RepoPagesDeploymentsHandler::cancel`].
    pub async fn cancel_deployment(
        &self,
        deployment_id: impl Into<PagesDeploymentId>,
    ) -> Result<()> {
        self.deployments().cancel(deployment_id).await
    }
}

/// A client to GitHub's repository pages builds API.
///
/// Created with [`RepoPagesHandler::builds`].
pub struct RepoPagesBuildsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoPagesBuildsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists builds of a GitHub Pages site.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#list-apiname-pages-builds)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let builds = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .builds()
    ///     .list()
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListPagesBuildsBuilder<'octo, 'r> {
        ListPagesBuildsBuilder::new(self.handler)
    }

    /// Gets the latest build of a GitHub Pages site.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-latest-pages-build)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let latest = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .builds()
    ///     .latest()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn latest(&self) -> Result<PageBuild> {
        let route = format!("/{}/pages/builds/latest", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Gets a specific build of a GitHub Pages site by ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-github-pages-build)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let build = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .builds()
    ///     .get(5472601u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, build_id: impl Into<PageBuildId>) -> Result<PageBuild> {
        let build_id = build_id.into();
        let route = format!("/{}/pages/builds/{build_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Requests a new GitHub Pages build.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#request-a-apiname-pages-build)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let build_status = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .builds()
    ///     .request()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request(&self) -> Result<PageBuildStatus> {
        let route = format!("/{}/pages/builds", self.handler.repo);
        self.handler.crab.post(route, None::<&()>).await
    }
}

/// A client to GitHub's repository pages deployments API.
///
/// Created with [`RepoPagesHandler::deployments`].
pub struct RepoPagesDeploymentsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoPagesDeploymentsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a builder to create a GitHub Pages deployment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#create-a-github-pages-deployment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let deployment = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .deployments()
    ///     .create("oidc_jwt_token")
    ///     .artifact_id(12345u64)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, oidc_token: impl Into<String>) -> CreatePagesDeploymentBuilder<'octo, 'r> {
        CreatePagesDeploymentBuilder::new(self.handler, oidc_token.into())
    }

    /// Gets the status of a GitHub Pages deployment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#get-the-status-of-a-github-pages-deployment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let status = octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .deployments()
    ///     .status("4fd754f7e594640989b406850d0bc8f06a121251")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn status(
        &self,
        deployment_id: impl Into<PagesDeploymentId>,
    ) -> Result<PagesDeploymentStatus> {
        let deployment_id = deployment_id.into();
        let route = format!("/{}/pages/deployments/{deployment_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Cancels a GitHub Pages deployment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#cancel-a-github-pages-deployment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .pages()
    ///     .deployments()
    ///     .cancel("4fd754f7e594640989b406850d0bc8f06a121251")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel(&self, deployment_id: impl Into<PagesDeploymentId>) -> Result<()> {
        let deployment_id = deployment_id.into();
        let route = format!(
            "/{}/pages/deployments/{deployment_id}/cancel",
            self.handler.repo
        );
        let response = self.handler.crab._post(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for creating a GitHub Pages site.
///
/// Created by [`RepoPagesHandler::create`].
#[derive(serde::Serialize)]
pub struct CreatePagesSiteBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_type: Option<PagesBuildType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PagesSource>,
}

impl<'octo, 'r> CreatePagesSiteBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            build_type: None,
            source: None,
        }
    }

    /// The process in which the Page will be built. Possible values are `legacy` and `workflow`.
    pub fn build_type(mut self, build_type: impl Into<PagesBuildType>) -> Self {
        self.build_type = Some(build_type.into());
        self
    }

    /// The source branch and directory used to publish your Pages site.
    pub fn source(mut self, source: PagesSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the source branch and path used to publish your Pages site.
    pub fn source_branch(mut self, branch: impl Into<String>, path: impl Into<String>) -> Self {
        self.source = Some(PagesSource::new(branch, path));
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<PagesSite> {
        let route = format!("/{}/pages", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating information about a GitHub Pages site.
///
/// Created by [`RepoPagesHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdatePagesSiteBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_enforced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_type: Option<PagesBuildType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<UpdatePagesSource>,
}

impl<'octo, 'r> UpdatePagesSiteBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            cname: None,
            https_enforced: None,
            build_type: None,
            source: None,
        }
    }

    /// Specify a custom domain for the repository.
    pub fn cname(mut self, cname: impl Into<String>) -> Self {
        self.cname = Some(Some(cname.into()));
        self
    }

    /// Removes the custom domain by sending `null`.
    pub fn remove_cname(mut self) -> Self {
        self.cname = Some(None);
        self
    }

    /// Specify whether HTTPS should be enforced for the repository.
    pub fn https_enforced(mut self, https_enforced: bool) -> Self {
        self.https_enforced = Some(https_enforced);
        self
    }

    /// The process by which the GitHub Pages site will be built.
    pub fn build_type(mut self, build_type: impl Into<PagesBuildType>) -> Self {
        self.build_type = Some(build_type.into());
        self
    }

    /// Update the source for the repository.
    pub fn source(mut self, source: impl Into<UpdatePagesSource>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<()> {
        let route = format!("/{}/pages", self.handler.repo);
        let response = self.handler.crab._put(route, Some(&self)).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for listing GitHub Pages builds.
///
/// Created by [`RepoPagesBuildsHandler::list`] or [`RepoPagesHandler::list_builds`].
#[derive(serde::Serialize)]
pub struct ListPagesBuildsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListPagesBuildsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100). Default: 30.
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
    pub async fn send(self) -> Result<Page<PageBuild>> {
        let route = format!("/{}/pages/builds", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a GitHub Pages deployment.
///
/// Created by [`RepoPagesDeploymentsHandler::create`] or [`RepoPagesHandler::create_deployment`].
#[derive(serde::Serialize)]
pub struct CreatePagesDeploymentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,
    pages_build_version: String,
    oidc_token: String,
}

impl<'octo, 'r> CreatePagesDeploymentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, oidc_token: String) -> Self {
        Self {
            handler,
            artifact_id: None,
            artifact_url: None,
            environment: None,
            pages_build_version: "GITHUB_SHA".to_string(),
            oidc_token,
        }
    }

    /// The ID of an artifact that contains the .zip or .tar of static assets to deploy.
    pub fn artifact_id(mut self, artifact_id: impl Into<u64>) -> Self {
        self.artifact_id = Some(artifact_id.into());
        self
    }

    /// The URL of an artifact that contains the .zip or .tar of static assets to deploy.
    pub fn artifact_url(mut self, artifact_url: impl Into<String>) -> Self {
        self.artifact_url = Some(artifact_url.into());
        self
    }

    /// The target environment for this GitHub Pages deployment (default: `github-pages`).
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// A unique string that represents the version of the build for this deployment (default: `GITHUB_SHA`).
    pub fn pages_build_version(mut self, pages_build_version: impl Into<String>) -> Self {
        self.pages_build_version = pages_build_version.into();
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<PagesDeployment> {
        let route = format!("/{}/pages/deployments", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}
