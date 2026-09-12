use crate::{actions::ActionsHandler, models::RunnerGroupId};
use serde::Serialize;

enum RunnerScope {
    Org(String),
    Repo { owner: String, repo: String },
}

/// A builder pattern struct for listing self-hosted runners.
///
/// Created by [`ActionsHandler::list_org_self_hosted_runners`] or
/// [`ActionsHandler::list_repo_self_hosted_runners`].
///
/// [`ActionsHandler::list_org_self_hosted_runners`]: ../struct.ActionsHandler.html#method.list_org_self_hosted_runners
/// [`ActionsHandler::list_repo_self_hosted_runners`]: ../struct.ActionsHandler.html#method.list_repo_self_hosted_runners
#[derive(Serialize)]
pub struct ListSelfHostedRunnersBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    #[serde(skip)]
    scope: RunnerScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListSelfHostedRunnersBuilder<'octo, 'r> {
    pub(crate) fn new_org(handler: &'r ActionsHandler<'octo>, org: String) -> Self {
        Self {
            handler,
            scope: RunnerScope::Org(org),
            name: None,
            per_page: None,
            page: None,
        }
    }

    pub(crate) fn new_repo(
        handler: &'r ActionsHandler<'octo>,
        owner: String,
        repo: String,
    ) -> Self {
        Self {
            handler,
            scope: RunnerScope::Repo { owner, repo },
            name: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter the list by runners matching the given name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
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
    pub async fn send(
        self,
    ) -> crate::Result<crate::Page<crate::models::actions::SelfHostedRunner>> {
        let route = match &self.scope {
            RunnerScope::Org(org) => format!("/orgs/{org}/actions/runners"),
            RunnerScope::Repo { owner, repo } => format!("/repos/{owner}/{repo}/actions/runners"),
        };

        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating just-in-time runner configurations.
///
/// Created by [`ActionsHandler::create_org_jit_runner_config`] or
/// [`ActionsHandler::create_repo_jit_runner_config`].
///
/// [`ActionsHandler::create_org_jit_runner_config`]: ../struct.ActionsHandler.html#method.create_org_jit_runner_config
/// [`ActionsHandler::create_repo_jit_runner_config`]: ../struct.ActionsHandler.html#method.create_repo_jit_runner_config
#[derive(Serialize)]
pub struct CreateJitRunnerConfigBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    #[serde(skip)]
    scope: RunnerScope,
    name: String,
    runner_group_id: RunnerGroupId,
    labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_folder: Option<String>,
}

impl<'octo, 'r> CreateJitRunnerConfigBuilder<'octo, 'r> {
    pub(crate) fn new_org(
        handler: &'r ActionsHandler<'octo>,
        org: String,
        name: String,
        runner_group_id: RunnerGroupId,
        labels: Vec<String>,
    ) -> Self {
        Self {
            handler,
            scope: RunnerScope::Org(org),
            name,
            runner_group_id,
            labels,
            work_folder: None,
        }
    }

    pub(crate) fn new_repo(
        handler: &'r ActionsHandler<'octo>,
        owner: String,
        repo: String,
        name: String,
        runner_group_id: RunnerGroupId,
        labels: Vec<String>,
    ) -> Self {
        Self {
            handler,
            scope: RunnerScope::Repo { owner, repo },
            name,
            runner_group_id,
            labels,
            work_folder: None,
        }
    }

    /// The working directory to be used for job execution, relative to the runner install directory.
    pub fn work_folder(mut self, work_folder: impl Into<String>) -> Self {
        self.work_folder = Some(work_folder.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::actions::SelfHostedRunnerJitConfig> {
        let route = match &self.scope {
            RunnerScope::Org(org) => format!("/orgs/{org}/actions/runners/generate-jitconfig"),
            RunnerScope::Repo { owner, repo } => {
                format!("/repos/{owner}/{repo}/actions/runners/generate-jitconfig")
            }
        };

        self.handler.crab.post(route, Some(&self)).await
    }
}

impl<'octo> ActionsHandler<'octo> {
    /// Lists runner applications that are supported by GitHub Actions for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#list-runner-applications-for-an-organization)
    pub async fn list_org_runner_downloads(
        &self,
        org: impl AsRef<str>,
    ) -> crate::Result<Vec<crate::models::actions::RunnerApplicationDownload>> {
        let route = format!("/orgs/{org}/actions/runners/downloads", org = org.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Lists runner applications that are supported by GitHub Actions for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#list-runner-applications-for-a-repository)
    pub async fn list_repo_runner_downloads(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> crate::Result<Vec<crate::models::actions::RunnerApplicationDownload>> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/downloads",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Lists labels for a self-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#list-labels-for-a-self-hosted-runner-for-an-organization)
    pub async fn list_org_runner_labels(
        &self,
        org: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}/labels",
            org = org.as_ref(),
            runner_id = runner_id,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Adds custom labels to a self-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#add-custom-labels-to-a-self-hosted-runner-for-an-organization)
    pub async fn add_org_runner_labels(
        &self,
        org: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
        labels: &[impl AsRef<str>],
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        #[derive(Serialize)]
        struct Body<'a> {
            labels: Vec<&'a str>,
        }
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}/labels",
            org = org.as_ref(),
            runner_id = runner_id,
        );
        let body = Body {
            labels: labels.iter().map(AsRef::as_ref).collect(),
        };
        self.crab.post(route, Some(&body)).await
    }

    /// Sets custom labels for a self-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#set-custom-labels-for-a-self-hosted-runner-for-an-organization)
    pub async fn set_org_runner_labels(
        &self,
        org: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
        labels: &[impl AsRef<str>],
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        #[derive(Serialize)]
        struct Body<'a> {
            labels: Vec<&'a str>,
        }
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}/labels",
            org = org.as_ref(),
            runner_id = runner_id,
        );
        let body = Body {
            labels: labels.iter().map(AsRef::as_ref).collect(),
        };
        self.crab.put(route, Some(&body)).await
    }

    /// Removes all custom labels from a self-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#remove-all-custom-labels-from-a-self-hosted-runner-for-an-organization)
    pub async fn remove_all_org_runner_labels(
        &self,
        org: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}/labels",
            org = org.as_ref(),
            runner_id = runner_id,
        );
        self.crab.delete(route, None::<&()>).await
    }

    /// Removes a custom label from a self-hosted runner for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#remove-a-custom-label-from-a-self-hosted-runner-for-an-organization)
    pub async fn remove_org_runner_label(
        &self,
        org: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
        name: impl AsRef<str>,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}/labels/{name}",
            org = org.as_ref(),
            runner_id = runner_id,
            name = name.as_ref(),
        );
        self.crab.delete(route, None::<&()>).await
    }

    /// Lists labels for a self-hosted runner for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#list-labels-for-a-self-hosted-runner-for-a-repository)
    pub async fn list_repo_runner_labels(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            runner_id = runner_id,
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Adds custom labels to a self-hosted runner for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#add-custom-labels-to-a-self-hosted-runner-for-a-repository)
    pub async fn add_repo_runner_labels(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
        labels: &[impl AsRef<str>],
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        #[derive(Serialize)]
        struct Body<'a> {
            labels: Vec<&'a str>,
        }
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            runner_id = runner_id,
        );
        let body = Body {
            labels: labels.iter().map(AsRef::as_ref).collect(),
        };
        self.crab.post(route, Some(&body)).await
    }

    /// Sets custom labels for a self-hosted runner for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#set-custom-labels-for-a-self-hosted-runner-for-a-repository)
    pub async fn set_repo_runner_labels(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
        labels: &[impl AsRef<str>],
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        #[derive(Serialize)]
        struct Body<'a> {
            labels: Vec<&'a str>,
        }
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            runner_id = runner_id,
        );
        let body = Body {
            labels: labels.iter().map(AsRef::as_ref).collect(),
        };
        self.crab.put(route, Some(&body)).await
    }

    /// Removes all custom labels from a self-hosted runner for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#remove-all-custom-labels-from-a-self-hosted-runner-for-a-repository)
    pub async fn remove_all_repo_runner_labels(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            runner_id = runner_id,
        );
        self.crab.delete(route, None::<&()>).await
    }

    /// Removes a custom label from a self-hosted runner for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/self-hosted-runners?apiVersion=2022-11-28#remove-a-custom-label-from-a-self-hosted-runner-for-a-repository)
    pub async fn remove_repo_runner_label(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: crate::models::RunnerId,
        name: impl AsRef<str>,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerLabelsList> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name}",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            runner_id = runner_id,
            name = name.as_ref(),
        );
        self.crab.delete(route, None::<&()>).await
    }
}
