//! Repository Source Imports API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28)

use crate::api::repos::RepoRef;
use crate::models::migrations::{
    Import, ImportAuthor, LargeFile, LfsPreference, MapCommitAuthor, StartImport, UpdateImport,
};
use crate::models::ImportAuthorId;
use crate::{Octocrab, Result};

/// Handler for GitHub's Repository Source Imports API.
///
/// Created with [`crate::api::repos::RepoHandler::import`].
pub struct RepoImportHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoImportHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Gets an import status for a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#get-an-import-status)
    pub async fn get(&self) -> Result<Import> {
        let route = format!("/{}/import", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Starts a source import to a GitHub repository using GitHub Importer.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#start-an-import)
    pub async fn start(&self, body: &StartImport) -> Result<Import> {
        let route = format!("/{}/import", self.repo);
        let resp = self.crab._put(route, Some(body)).await?;
        let resp = crate::map_github_error(resp).await?;
        <Import as crate::FromResponse>::from_response(resp).await
    }

    /// Updates an import for a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#update-an-import)
    pub async fn update(&self, body: &UpdateImport) -> Result<Import> {
        let route = format!("/{}/import", self.repo);
        self.crab.patch(route, Some(body)).await
    }

    /// Cancels an ongoing import for a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#cancel-an-import)
    pub async fn cancel(&self) -> Result<()> {
        let route = format!("/{}/import", self.repo);
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Gets commit authors for the repository import.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#get-commit-authors)
    pub async fn authors(&self, since: Option<i64>) -> Result<Vec<ImportAuthor>> {
        let route = format!("/{}/import/authors", self.repo);
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            since: Option<i64>,
        }
        let query = Query { since };
        self.crab.get(route, Some(&query)).await
    }

    /// Maps a commit author identity for the repository import.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#map-a-commit-author)
    pub async fn map_author(
        &self,
        author_id: impl Into<ImportAuthorId>,
        body: &MapCommitAuthor,
    ) -> Result<ImportAuthor> {
        let route = format!("/{}/import/authors/{}", self.repo, author_id.into());
        self.crab.patch(route, Some(body)).await
    }

    /// Gets large files for the repository import.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#get-large-files)
    pub async fn large_files(&self) -> Result<Vec<LargeFile>> {
        let route = format!("/{}/import/large_files", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Updates Git LFS preference for the repository import.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/source-imports?apiVersion=2022-11-28#update-git-lfs-preference)
    pub async fn set_lfs_preference(&self, preference: LfsPreference) -> Result<Import> {
        let route = format!("/{}/import/lfs", self.repo);
        let body = serde_json::json!({
            "use_lfs": preference,
        });
        self.crab.patch(route, Some(&body)).await
    }
}
