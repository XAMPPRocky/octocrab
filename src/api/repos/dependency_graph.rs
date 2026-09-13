//! Repository Dependency Graph API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph?apiVersion=2022-11-28)

use crate::api::repos::sbom::RepoSbomHandler;
use crate::api::repos::RepoRef;
#[allow(deprecated)]
use crate::models::dependency_graph::Sbom;
use crate::models::dependency_graph::{CreateSnapshotResponse, DependencyDiff, Snapshot};
use crate::{Octocrab, Result};

/// Client for GitHub's repository Dependency Graph API.
///
/// Created with [`crate::api::repos::RepoHandler::dependency_graph`].
pub struct RepoDependencyGraphHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoDependencyGraphHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Gets the diff of the dependency changes between two commits of a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph/dependency-review?apiVersion=2022-11-28#get-a-diff-of-the-dependencies-between-commits)
    pub fn compare(&self, basehead: impl Into<String>) -> CompareDependenciesBuilder<'octo, '_> {
        CompareDependenciesBuilder::new(self, basehead.into())
    }

    /// Create a snapshot of dependencies for a repository.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph/dependency-submission?apiVersion=2022-11-28#create-a-snapshot-of-dependencies-for-a-repository)
    pub async fn create_snapshot(&self, snapshot: &Snapshot) -> Result<CreateSnapshotResponse> {
        let route = format!("/{}/dependency-graph/snapshots", self.repo);
        self.crab.post(route, Some(snapshot)).await
    }

    /// Exports the software bill of materials (SBOM) for a repository in SPDX JSON format.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph/sboms?apiVersion=2022-11-28#export-a-software-bill-of-materials-sbom-for-a-repository)
    #[deprecated(
        note = "This operation is closing down and will not be accessible after November 13, 2026. Please migrate to the asynchronous flow using generate_report and fetch_report."
    )]
    #[allow(deprecated)]
    pub async fn export_sbom(&self) -> Result<Sbom> {
        let route = format!("/{}/dependency-graph/sbom", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Access SBOM report generation and fetching APIs.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph/sboms?apiVersion=2022-11-28)
    pub fn sbom(&self) -> RepoSbomHandler<'octo> {
        RepoSbomHandler::new(self.crab, self.repo.clone())
    }
}

/// A builder pattern struct for comparing dependencies between two commits.
#[derive(serde::Serialize)]
pub struct CompareDependenciesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoDependencyGraphHandler<'octo>,
    #[serde(skip)]
    basehead: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl<'octo, 'r> CompareDependenciesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoDependencyGraphHandler<'octo>, basehead: String) -> Self {
        Self {
            handler,
            basehead,
            name: None,
        }
    }

    /// The full path, relative to the repository root, of the dependency manifest file.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Vec<DependencyDiff>> {
        let route = format!(
            "/{}/dependency-graph/compare/{}",
            self.handler.repo, self.basehead
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
