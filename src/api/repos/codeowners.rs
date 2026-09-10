use super::RepoHandler;
use crate::models::repos::CodeownersErrors;

/// A builder pattern struct for listing CODEOWNERS errors in a repository.
///
/// Created by [`RepoHandler::codeowners_errors`].
#[derive(serde::Serialize)]
pub struct ListCodeownersErrorsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
}

impl<'octo, 'r> ListCodeownersErrorsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            r#ref: None,
        }
    }

    /// A branch, tag, or commit name. Default is the repository's default branch.
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<CodeownersErrors> {
        let route = format!("/{}/codeowners/errors", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
