use super::RepoHandler;
use crate::models::{
    repos::{Deployment, DeploymentStatus, DeploymentStatusState},
    DeploymentId, DeploymentStatusId,
};
use crate::{Page, Result};

/// A client to GitHub's repository deployments API.
///
/// Created with [`RepoHandler::deployments`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/deployments?apiVersion=2022-11-28)
pub struct RepoDeploymentsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoDeploymentsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists deployments for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/deployments?apiVersion=2022-11-28#list-deployments)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let deployments = octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .list()
    ///     .environment("production")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListDeploymentsBuilder<'octo, 'r> {
        ListDeploymentsBuilder::new(self.handler)
    }

    /// Gets a single deployment by its ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/deployments?apiVersion=2022-11-28#get-a-deployment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let deployment = octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .get(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, deployment_id: impl Into<DeploymentId>) -> Result<Deployment> {
        let deployment_id = deployment_id.into();
        let route = format!("/{}/deployments/{deployment_id}", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a [`CreateDeploymentBuilder`] to configure a new deployment for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/deployments?apiVersion=2022-11-28#create-a-deployment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let deployment = octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .create("main")
    ///     .environment("production")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, r#ref: impl Into<String>) -> CreateDeploymentBuilder<'octo, 'r> {
        CreateDeploymentBuilder::new(self.handler, r#ref.into())
    }

    /// Deletes a deployment from a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/deployments?apiVersion=2022-11-28#delete-a-deployment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .delete(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, deployment_id: impl Into<DeploymentId>) -> Result<()> {
        let deployment_id = deployment_id.into();
        let route = format!("/{}/deployments/{deployment_id}", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access the deployment statuses API for a specific deployment.
    pub fn statuses(
        &self,
        deployment_id: impl Into<DeploymentId>,
    ) -> DeploymentStatusesHandler<'octo, 'r> {
        DeploymentStatusesHandler::new(self.handler, deployment_id.into())
    }
}

/// A client to GitHub's deployment statuses API for a specific deployment.
///
/// Created with [`RepoDeploymentsHandler::statuses`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/statuses?apiVersion=2022-11-28)
pub struct DeploymentStatusesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    deployment_id: DeploymentId,
}

impl<'octo, 'r> DeploymentStatusesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, deployment_id: DeploymentId) -> Self {
        Self {
            handler,
            deployment_id,
        }
    }

    /// Lists deployment statuses for a deployment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/statuses?apiVersion=2022-11-28#list-deployment-statuses)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let statuses = octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .statuses(42)
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListDeploymentStatusesBuilder<'octo, 'r> {
        ListDeploymentStatusesBuilder::new(self.handler, self.deployment_id)
    }

    /// Gets a single deployment status by its ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/statuses?apiVersion=2022-11-28#get-a-deployment-status)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let status = octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .statuses(42)
    ///     .get(1)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, status_id: impl Into<DeploymentStatusId>) -> Result<DeploymentStatus> {
        let status_id = status_id.into();
        let route = format!(
            "/{}/deployments/{}/statuses/{status_id}",
            self.handler.repo, self.deployment_id
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a [`CreateDeploymentStatusBuilder`] to create a new status for a deployment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/statuses?apiVersion=2022-11-28#create-a-deployment-status)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use octocrab::models::repos::DeploymentStatusState;
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let status = octocrab.repos("owner", "repo")
    ///     .deployments()
    ///     .statuses(42)
    ///     .create(DeploymentStatusState::Success)
    ///     .environment("production")
    ///     .description("Deployment finished successfully.")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, state: DeploymentStatusState) -> CreateDeploymentStatusBuilder<'octo, 'r> {
        CreateDeploymentStatusBuilder::new(self.handler, self.deployment_id, state)
    }
}

/// A builder pattern struct for listing deployments for a repository.
///
/// Created by [`RepoDeploymentsHandler::list`].
#[derive(serde::Serialize)]
pub struct ListDeploymentsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListDeploymentsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            sha: None,
            r#ref: None,
            task: None,
            environment: None,
            per_page: None,
            page: None,
        }
    }

    /// The SHA recorded at creation time.
    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    /// The name of the ref (branch, tag, or SHA).
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// The name of the task for the deployment (e.g., deploy or deploy:migrations).
    pub fn task(mut self, task: impl Into<String>) -> Self {
        self.task = Some(task.into());
        self
    }

    /// The name of the environment that was deployed to (e.g., staging or production).
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<Deployment>> {
        let route = format!("/{}/deployments", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a deployment for a repository.
///
/// Created by [`RepoDeploymentsHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateDeploymentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(rename = "ref")]
    r#ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_contexts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transient_environment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    production_environment: Option<bool>,
}

impl<'octo, 'r> CreateDeploymentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, r#ref: String) -> Self {
        Self {
            handler,
            r#ref,
            task: None,
            auto_merge: None,
            required_contexts: None,
            payload: None,
            environment: None,
            description: None,
            transient_environment: None,
            production_environment: None,
        }
    }

    /// Specifies a task to execute (e.g., deploy or deploy:migrations). Default: "deploy".
    pub fn task(mut self, task: impl Into<String>) -> Self {
        self.task = Some(task.into());
        self
    }

    /// Attempts to automatically merge the default branch into the requested ref, if it's behind the default branch. Default: true.
    pub fn auto_merge(mut self, auto_merge: bool) -> Self {
        self.auto_merge = Some(auto_merge);
        self
    }

    /// The status contexts to verify against commit status checks. To bypass checking entirely, pass an empty array.
    pub fn required_contexts(
        mut self,
        contexts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.required_contexts = Some(contexts.into_iter().map(Into::into).collect());
        self
    }

    /// JSON payload with extra information about the deployment.
    pub fn payload(mut self, payload: impl Into<serde_json::Value>) -> Self {
        self.payload = Some(payload.into());
        self
    }

    /// Name for the target deployment environment (e.g., production, staging, qa). Default: "production".
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Short description of the deployment.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Specifies if the given environment is specific to the deployment and will no longer exist in the future. Default: false.
    pub fn transient_environment(mut self, transient: bool) -> Self {
        self.transient_environment = Some(transient);
        self
    }

    /// Specifies if the given environment is one that end-users directly interact with. Default: true when environment is production and false otherwise.
    pub fn production_environment(mut self, production: bool) -> Self {
        self.production_environment = Some(production);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Deployment> {
        let route = format!("/{}/deployments", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}

/// A builder pattern struct for listing deployment statuses for a deployment.
///
/// Created by [`DeploymentStatusesHandler::list`].
#[derive(serde::Serialize)]
pub struct ListDeploymentStatusesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    deployment_id: DeploymentId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListDeploymentStatusesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, deployment_id: DeploymentId) -> Self {
        Self {
            handler,
            deployment_id,
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
    pub async fn send(self) -> Result<Page<DeploymentStatus>> {
        let route = format!(
            "/{}/deployments/{}/statuses",
            self.handler.repo, self.deployment_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a deployment status for a deployment.
///
/// Created by [`DeploymentStatusesHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateDeploymentStatusBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    deployment_id: DeploymentId,
    state: DeploymentStatusState,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_inactive: Option<bool>,
}

impl<'octo, 'r> CreateDeploymentStatusBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        deployment_id: DeploymentId,
        state: DeploymentStatusState,
    ) -> Self {
        Self {
            handler,
            deployment_id,
            state,
            target_url: None,
            log_url: None,
            description: None,
            environment: None,
            environment_url: None,
            auto_inactive: None,
        }
    }

    /// The target URL to associate with this status (legacy, prefer `log_url`).
    pub fn target_url(mut self, target_url: impl Into<String>) -> Self {
        self.target_url = Some(target_url.into());
        self
    }

    /// The full URL of the deployment's output. Recommended over `target_url`.
    pub fn log_url(mut self, log_url: impl Into<String>) -> Self {
        self.log_url = Some(log_url.into());
        self
    }

    /// A short description of the status (maximum 140 characters).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Name for the target deployment environment.
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Sets the URL for accessing your environment.
    pub fn environment_url(mut self, environment_url: impl Into<String>) -> Self {
        self.environment_url = Some(environment_url.into());
        self
    }

    /// Adds a new inactive status to all prior non-transient, non-production deployments with the same environment. Default: true.
    pub fn auto_inactive(mut self, auto_inactive: bool) -> Self {
        self.auto_inactive = Some(auto_inactive);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<DeploymentStatus> {
        let route = format!(
            "/{}/deployments/{}/statuses",
            self.handler.repo, self.deployment_id
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}
