use super::*;

#[derive(serde::Serialize)]
pub struct UpdateFileBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: String,
    message: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    /// The branch to commit to.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// The person that commited the file.
    pub fn commiter(mut self, commiter: impl Into<models::AuthorUser>) -> Self {
        self.commiter = Some(commiter.into());
        self
    }

    /// The author of the file.
    pub fn author(mut self, author: impl Into<models::AuthorUser>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::FileUpdate> {
        let url = format!(
            "/repos/{owner}/{repo}/contents/{path}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            path = self.path,
        );
        self.handler.crab.put(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::AuthorUser;

    #[test]
    fn serialize() {
        let octocrab = crate::instance();
        let repo = octocrab.repos("owner", "repo");
        let builder = repo
            .update_file(
                "tests/test.txt",
                "Update test.txt",
                "This is a test.",
                "testsha",
            )
            .branch("not-master")
            .commiter(AuthorUser {
                name: "Octocat".to_string(),
                email: "octocat@github.com".to_string(),
            })
            .author(AuthorUser {
                name: "Ferris".to_string(),
                email: "ferris@rust-lang.org".to_string(),
            });

        assert_eq!(
            serde_json::to_value(builder).unwrap(),
            serde_json::json!({
                "message": "Update test.txt",
                "content": base64::encode("This is a test."),
                "sha": "testsha",
                "branch": "not-master",
                "commiter": {
                    "name": "Octocat",
                    "email": "octocat@github.com"
                },
                "author": {
                    "name": "Ferris",
                    "email": "ferris@rust-lang.org"
                }
            })
        )
    }
}
