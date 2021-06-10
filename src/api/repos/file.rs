use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct UpdateFileBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: String,
    message: String,
    content: String,
    sha: Option<String>,
    /// The branch to commit to.
    branch: Option<String>,
    /// The person that commited the file.
    commiter: Option<models::repos::GitUser>,
    /// The author of the file.
    author: Option<models::repos::GitUser>,
}

impl<'octo, 'r> UpdateFileBuilder<'octo, 'r> {
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
