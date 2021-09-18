use super::*;

#[derive(serde::Serialize)]
pub struct GetContentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
}

impl<'octo, 'r> GetContentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            path: None,
            r#ref: None,
        }
    }

    /// The content path.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// The name of the commit/branch/tag.
    /// Default: the repository’s default branch (usually `master)
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::ContentItems> {
        let path = self.path.clone().unwrap_or(String::from(""));
        let url = format!(
            "repos/{owner}/{repo}/contents/{path}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            path = path,
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

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
    commiter: Option<models::repos::GitUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<models::repos::GitUser>,
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
    pub fn commiter(mut self, commiter: impl Into<models::repos::GitUser>) -> Self {
        self.commiter = Some(commiter.into());
        self
    }

    /// The author of the file.
    pub fn author(mut self, author: impl Into<models::repos::GitUser>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::FileUpdate> {
        let url = format!(
            "repos/{owner}/{repo}/contents/{path}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            path = self.path,
        );
        self.handler.crab.put(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::repos::GitUser;

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
            .commiter(GitUser {
                name: "Octocat".to_string(),
                email: "octocat@github.com".to_string(),
            })
            .author(GitUser {
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
