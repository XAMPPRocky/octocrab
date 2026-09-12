use super::RepoHandler;
use crate::Result;

/// A client to GitHub's repository dispatches API.
///
/// Created with [`RepoHandler::dispatches`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-dispatch-event)
pub struct RepoDispatchesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoDispatchesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`CreateDispatchBuilder`] to trigger a `repository_dispatch` webhook event.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-dispatch-event)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.repos("owner", "repo")
    ///     .dispatches()
    ///     .create("on-demand-test")
    ///     .client_payload(serde_json::json!({"unit": false, "integration": true}))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, event_type: impl Into<String>) -> CreateDispatchBuilder<'octo, 'r> {
        CreateDispatchBuilder::new(self.handler, event_type.into())
    }
}

/// A builder pattern struct for triggering a repository dispatch event.
///
/// Created by [`RepoDispatchesHandler::create`], [`RepoHandler::create_dispatch`], or [`RepoHandler::dispatch`].
#[derive(serde::Serialize)]
pub struct CreateDispatchBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_payload: Option<serde_json::Value>,
}

impl<'octo, 'r> CreateDispatchBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, event_type: String) -> Self {
        Self {
            handler,
            event_type,
            client_payload: None,
        }
    }

    /// An optional JSON payload with extra information about the webhook event.
    ///
    /// The maximum number of top-level properties is 10. The total size of the JSON payload must be less than 64KB.
    pub fn client_payload(mut self, client_payload: impl Into<serde_json::Value>) -> Self {
        self.client_payload = Some(client_payload.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<()> {
        let route = format!("/{}/dispatches", self.handler.repo);
        let response = self.handler.crab._post(route, Some(&self)).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
