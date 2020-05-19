//! The repositories API.

mod file;

pub use file::UpdateFileBuilder;

use crate::{models, params, Octocrab, Result};

pub struct RepoHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> RepoHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    pub async fn get_ref(&self, reference: &params::repos::Reference) -> Result<models::Ref> {
        let url = format!(
            "/repos/{owner}/{repo}/git/ref/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(url, None::<&()>).await
    }

    pub async fn create_ref(
        &self,
        reference: &params::repos::Reference,
        sha: String,
    ) -> Result<models::Ref> {
        let url = format!(
            "/repos/{owner}/{repo}/git/refs",
            owner = self.owner,
            repo = self.repo,
        );
        self.crab
            .post(
                url,
                Some(&serde_json::json!({
                    "ref": reference.full_ref_url(),
                    "sha": sha,
                })),
            )
            .await
    }

    pub fn create_file(
        &self,
        path: impl Into<String>,
        message: impl Into<String>,
        content: impl AsRef<[u8]>,
    ) -> UpdateFileBuilder<'_, '_> {
        UpdateFileBuilder::new(
            self,
            path.into(),
            message.into(),
            base64::encode(content),
            None,
        )
    }

    pub fn update_file(
        &self,
        path: impl Into<String>,
        message: impl Into<String>,
        content: impl AsRef<[u8]>,
        sha: impl Into<String>,
    ) -> UpdateFileBuilder<'_, '_> {
        UpdateFileBuilder::new(
            self,
            path.into(),
            message.into(),
            base64::encode(content),
            Some(sha.into()),
        )
    }

    pub fn pulls(&self) -> super::pulls::PullRequestHandler {
        super::pulls::PullRequestHandler::new(self.crab, self.owner.clone(), self.repo.clone())
    }
}
