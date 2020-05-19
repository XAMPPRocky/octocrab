use super::*;

#[derive(serde::Serialize)]
pub struct UpdateFileBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: String,
    message: String,
    content: String,
    sha: Option<String>,
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commiter: Option<models::AuthorUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<models::AuthorUser>,
}

impl<'octo, 'r> UpdateFileBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        path: String,
        message: String,
        content: String,
        sha: Option<String>,
    ) -> Self {
        Self {
            handler,
            path,
            message,
            content,
            sha,
            branch: None,
            commiter: None,
            author: None,
        }
    }

    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    pub fn commiter(mut self, commiter: impl Into<models::AuthorUser>) -> Self {
        self.commiter = Some(commiter.into());
        self
    }

    pub fn author(mut self, author: impl Into<models::AuthorUser>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub async fn send(self) -> Result<serde_json::Value> {
        // FIXME: change return type
        let url = format!(
            "/repos/{owner}/{repo}/contents/{path}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            path = self.path,
        );
        self.handler.crab.put(url, None::<&()>, Some(&self)).await
    }
}
