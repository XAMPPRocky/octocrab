use serde::Serialize;

use crate::models::agents::{
    AgentAutomation, AgentAutomationsResponse, AgentAvailableTool, AgentAvailableTrigger,
    AgentPublicKey, AgentRepositoriesResponse, AgentSecret, AgentSecretsResponse, AgentTask,
    AgentTasksResponse, AgentVariable, AgentVariablesResponse,
};
use crate::{Octocrab, Result};

/// A client to GitHub's Agents API.
///
/// Created with [`Octocrab::agents`].
pub struct AgentsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> AgentsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Access agent automations API.
    pub fn automations(&self) -> AgentAutomationsHandler<'octo> {
        AgentAutomationsHandler { crab: self.crab }
    }

    /// Access agent tasks API.
    pub fn tasks(&self) -> AgentTasksHandler<'octo> {
        AgentTasksHandler { crab: self.crab }
    }
}

/// Handler for agent automations.
pub struct AgentAutomationsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> AgentAutomationsHandler<'octo> {
    /// List automations for user.
    pub async fn list(&self) -> Result<Vec<AgentAutomation>> {
        self.crab.get("/agents/automations", None::<&()>).await
    }

    /// List available automation tools.
    pub async fn get_available_tools(&self) -> Result<Vec<AgentAvailableTool>> {
        self.crab
            .get("/agents/automations/available-tools", None::<&()>)
            .await
    }

    /// List available automation triggers.
    pub async fn get_available_triggers(&self) -> Result<Vec<AgentAvailableTrigger>> {
        self.crab
            .get("/agents/automations/available-triggers", None::<&()>)
            .await
    }

    /// Get an automation by ID.
    pub async fn get(&self, automation_id: impl AsRef<str>) -> Result<AgentAutomation> {
        let route = format!("/agents/automations/{}", automation_id.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// List tasks for an automation.
    pub async fn list_tasks(&self, automation_id: impl AsRef<str>) -> Result<AgentTasksResponse> {
        let route = format!("/agents/automations/{}/tasks", automation_id.as_ref());
        self.crab.get(route, None::<&()>).await
    }
}

/// Handler for agent tasks.
pub struct AgentTasksHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> AgentTasksHandler<'octo> {
    /// List agent tasks.
    pub async fn list(&self) -> Result<AgentTasksResponse> {
        self.crab.get("/agents/tasks", None::<&()>).await
    }

    /// Get a task by ID.
    pub async fn get(&self, task_id: impl AsRef<str>) -> Result<AgentTask> {
        let route = format!("/agents/tasks/{}", task_id.as_ref());
        self.crab.get(route, None::<&()>).await
    }
}

// --------------------------- Repository Agents Handlers ---------------------------

use crate::api::repos::RepoHandler;

/// Handler for repository-scoped agent operations.
pub struct RepoAgentsHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoAgentsHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    pub fn automations(&self) -> RepoAgentAutomationsHandler<'octo, 'b> {
        RepoAgentAutomationsHandler {
            handler: self.handler,
        }
    }

    pub fn tasks(&self) -> RepoAgentTasksHandler<'octo, 'b> {
        RepoAgentTasksHandler {
            handler: self.handler,
        }
    }

    pub fn secrets(&self) -> RepoAgentSecretsHandler<'octo, 'b> {
        RepoAgentSecretsHandler {
            handler: self.handler,
        }
    }

    pub fn variables(&self) -> RepoAgentVariablesHandler<'octo, 'b> {
        RepoAgentVariablesHandler {
            handler: self.handler,
        }
    }
}

pub struct RepoAgentAutomationsHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoAgentAutomationsHandler<'octo, 'b> {
    /// List automations for a repository.
    pub async fn list(&self) -> Result<AgentAutomationsResponse> {
        let route = format!("/agents/{}/automations", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create an automation for a repository.
    pub async fn create(&self, body: &impl Serialize) -> Result<AgentAutomation> {
        let route = format!("/agents/{}/automations", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get an automation for a repository.
    pub async fn get(&self, automation_id: impl AsRef<str>) -> Result<AgentAutomation> {
        let route = format!(
            "/agents/{}/automations/{}",
            self.handler.repo,
            automation_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Edit an automation for a repository.
    pub async fn update(
        &self,
        automation_id: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<AgentAutomation> {
        let route = format!(
            "/agents/{}/automations/{}",
            self.handler.repo,
            automation_id.as_ref()
        );
        self.handler.crab.patch(route, Some(body)).await
    }

    /// Delete an automation for a repository.
    pub async fn delete(&self, automation_id: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/agents/{}/automations/{}",
            self.handler.repo,
            automation_id.as_ref()
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

pub struct RepoAgentTasksHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoAgentTasksHandler<'octo, 'b> {
    /// List tasks for repository.
    pub async fn list(&self) -> Result<AgentTasksResponse> {
        let route = format!("/agents/{}/tasks", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Start a task.
    pub async fn start(&self, body: &impl Serialize) -> Result<AgentTask> {
        let route = format!("/agents/{}/tasks", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get a task by repo.
    pub async fn get(&self, task_id: impl AsRef<str>) -> Result<AgentTask> {
        let route = format!("/agents/{}/tasks/{}", self.handler.repo, task_id.as_ref());
        self.handler.crab.get(route, None::<&()>).await
    }
}

pub struct RepoAgentSecretsHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoAgentSecretsHandler<'octo, 'b> {
    /// List repository organization secrets.
    pub async fn list_org_secrets(&self) -> Result<AgentSecretsResponse> {
        let route = format!("/{}/agents/organization-secrets", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// List repository secrets.
    pub async fn list(&self) -> Result<AgentSecretsResponse> {
        let route = format!("/{}/agents/secrets", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get a repository public key.
    pub async fn get_public_key(&self) -> Result<AgentPublicKey> {
        let route = format!("/{}/agents/secrets/public-key", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Get a repository secret.
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<AgentSecret> {
        let route = format!(
            "/{}/agents/secrets/{}",
            self.handler.repo,
            secret_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create or update a repository secret.
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/{}/agents/secrets/{}",
            self.handler.repo,
            secret_name.as_ref()
        );
        let res = self.handler.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Delete a repository secret.
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/{}/agents/secrets/{}",
            self.handler.repo,
            secret_name.as_ref()
        );
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

pub struct RepoAgentVariablesHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoAgentVariablesHandler<'octo, 'b> {
    /// List repository organization variables.
    pub async fn list_org_variables(&self) -> Result<AgentVariablesResponse> {
        let route = format!("/{}/agents/organization-variables", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// List repository variables.
    pub async fn list(&self) -> Result<AgentVariablesResponse> {
        let route = format!("/{}/agents/variables", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create a repository variable.
    pub async fn create(&self, body: &impl Serialize) -> Result<AgentVariable> {
        let route = format!("/{}/agents/variables", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get a repository variable.
    pub async fn get(&self, name: impl AsRef<str>) -> Result<AgentVariable> {
        let route = format!("/{}/agents/variables/{}", self.handler.repo, name.as_ref());
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a repository variable.
    pub async fn update(&self, name: impl AsRef<str>, body: &impl Serialize) -> Result<()> {
        let route = format!("/{}/agents/variables/{}", self.handler.repo, name.as_ref());
        let res = self.handler.crab._patch(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Delete a repository variable.
    pub async fn delete(&self, name: impl AsRef<str>) -> Result<()> {
        let route = format!("/{}/agents/variables/{}", self.handler.repo, name.as_ref());
        let res = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

// --------------------------- Organization Agents Handlers ---------------------------

/// Handler for organization-scoped agent operations.
pub struct OrgAgentsHandler<'octo, 'b> {
    crab: &'octo Octocrab,
    org: &'b str,
}

impl<'octo, 'b> OrgAgentsHandler<'octo, 'b> {
    pub(crate) fn new(crab: &'octo Octocrab, org: &'b str) -> Self {
        Self { crab, org }
    }

    pub fn secrets(&self) -> OrgAgentSecretsHandler<'octo, 'b> {
        OrgAgentSecretsHandler {
            crab: self.crab,
            org: self.org,
        }
    }

    pub fn variables(&self) -> OrgAgentVariablesHandler<'octo, 'b> {
        OrgAgentVariablesHandler {
            crab: self.crab,
            org: self.org,
        }
    }
}

pub struct OrgAgentSecretsHandler<'octo, 'b> {
    crab: &'octo Octocrab,
    org: &'b str,
}

impl<'octo, 'b> OrgAgentSecretsHandler<'octo, 'b> {
    /// List organization secrets.
    pub async fn list(&self) -> Result<AgentSecretsResponse> {
        let route = format!("/orgs/{}/agents/secrets", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Get an organization public key.
    pub async fn get_public_key(&self) -> Result<AgentPublicKey> {
        let route = format!("/orgs/{}/agents/secrets/public-key", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Get an organization secret.
    pub async fn get(&self, secret_name: impl AsRef<str>) -> Result<AgentSecret> {
        let route = format!("/orgs/{}/agents/secrets/{}", self.org, secret_name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Create or update an organization secret.
    pub async fn create_or_update(
        &self,
        secret_name: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!("/orgs/{}/agents/secrets/{}", self.org, secret_name.as_ref());
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Delete an organization secret.
    pub async fn delete(&self, secret_name: impl AsRef<str>) -> Result<()> {
        let route = format!("/orgs/{}/agents/secrets/{}", self.org, secret_name.as_ref());
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List selected repositories for an organization secret.
    pub async fn list_repositories(
        &self,
        secret_name: impl AsRef<str>,
    ) -> Result<AgentRepositoriesResponse> {
        let route = format!(
            "/orgs/{}/agents/secrets/{}/repositories",
            self.org,
            secret_name.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set selected repositories for an organization secret.
    pub async fn set_repositories(
        &self,
        secret_name: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/agents/secrets/{}/repositories",
            self.org,
            secret_name.as_ref()
        );
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Add selected repository to an organization secret.
    pub async fn add_repository(
        &self,
        secret_name: impl AsRef<str>,
        repository_id: u64,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/agents/secrets/{}/repositories/{}",
            self.org,
            secret_name.as_ref(),
            repository_id
        );
        let res = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Remove selected repository from an organization secret.
    pub async fn remove_repository(
        &self,
        secret_name: impl AsRef<str>,
        repository_id: u64,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/agents/secrets/{}/repositories/{}",
            self.org,
            secret_name.as_ref(),
            repository_id
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}

pub struct OrgAgentVariablesHandler<'octo, 'b> {
    crab: &'octo Octocrab,
    org: &'b str,
}

impl<'octo, 'b> OrgAgentVariablesHandler<'octo, 'b> {
    /// List organization variables.
    pub async fn list(&self) -> Result<AgentVariablesResponse> {
        let route = format!("/orgs/{}/agents/variables", self.org);
        self.crab.get(route, None::<&()>).await
    }

    /// Create an organization variable.
    pub async fn create(&self, body: &impl Serialize) -> Result<AgentVariable> {
        let route = format!("/orgs/{}/agents/variables", self.org);
        self.crab.post(route, Some(body)).await
    }

    /// Get an organization variable.
    pub async fn get(&self, name: impl AsRef<str>) -> Result<AgentVariable> {
        let route = format!("/orgs/{}/agents/variables/{}", self.org, name.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Update an organization variable.
    pub async fn update(&self, name: impl AsRef<str>, body: &impl Serialize) -> Result<()> {
        let route = format!("/orgs/{}/agents/variables/{}", self.org, name.as_ref());
        let res = self.crab._patch(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Delete an organization variable.
    pub async fn delete(&self, name: impl AsRef<str>) -> Result<()> {
        let route = format!("/orgs/{}/agents/variables/{}", self.org, name.as_ref());
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List selected repositories for an organization variable.
    pub async fn list_repositories(
        &self,
        name: impl AsRef<str>,
    ) -> Result<AgentRepositoriesResponse> {
        let route = format!(
            "/orgs/{}/agents/variables/{}/repositories",
            self.org,
            name.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set selected repositories for an organization variable.
    pub async fn set_repositories(
        &self,
        name: impl AsRef<str>,
        body: &impl Serialize,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/agents/variables/{}/repositories",
            self.org,
            name.as_ref()
        );
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Add selected repository to an organization variable.
    pub async fn add_repository(&self, name: impl AsRef<str>, repository_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/agents/variables/{}/repositories/{}",
            self.org,
            name.as_ref(),
            repository_id
        );
        let res = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Remove selected repository from an organization variable.
    pub async fn remove_repository(&self, name: impl AsRef<str>, repository_id: u64) -> Result<()> {
        let route = format!(
            "/orgs/{}/agents/variables/{}/repositories/{}",
            self.org,
            name.as_ref(),
            repository_id
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }
}
