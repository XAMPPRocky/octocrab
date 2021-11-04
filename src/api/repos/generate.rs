use crate::{Error, repos::RepoHandler};

#[derive(serde::Serialize)]
pub struct GenerateRepositoryBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all_branches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
}

impl<'octo, 'r> GenerateRepositoryBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, name: impl Into<String>) -> Self {
        Self {
            handler,
            name: name.into(),
            owner: None,
            description: None,
            include_all_branches: None,
            private: None,
        }
    }

    /// New owner of the newly created repository from selected template.
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Description of the newly created repository.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Whether to include all branches from template repository.
    pub fn include_all_branches(mut self, include_all_branches: impl Into<bool>) -> Self {
        self.include_all_branches = Some(include_all_branches.into());
        self
    }

    /// Whether to set newly created repository to private .
    pub fn private(mut self, private: impl Into<bool>) -> Self {
        self.private = Some(private.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<(), Error> {
        let url = format!(
            "repos/{owner}/{repo}/generate",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        let request = self.handler
            .crab
            .client
            .post(self.handler.crab.absolute_url(url)?)
            .body(serde_json::to_string(&self).unwrap())
            .header(reqwest::header::ACCEPT, "application/vnd.github.baptiste-preview+json");

        let response = self.handler.crab.execute(request).await?;
        crate::map_github_error(response)
            .await
            .map(drop)
    }
}