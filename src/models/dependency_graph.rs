//! Data models for GitHub Dependency Graph.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph?apiVersion=2022-11-28)

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use super::repos::sbom::SbomDependencyGraph;

/// Type of change in a dependency diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Added,
    Removed,
}

/// Scope of a dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyScope {
    Unknown,
    Runtime,
    Development,
}

/// Relationship of a dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRelationship {
    Direct,
    Indirect,
}

/// A vulnerability in a dependency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependencyVulnerability {
    pub severity: String,
    pub advisory_ghsa_id: String,
    pub advisory_summary: String,
    pub advisory_url: Url,
}

/// A diff of a dependency between two commits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependencyDiff {
    pub change_type: ChangeType,
    pub manifest: String,
    pub ecosystem: String,
    pub name: String,
    pub version: String,
    pub package_url: Option<String>,
    pub license: Option<String>,
    pub source_repository_url: Option<Url>,
    pub vulnerabilities: Vec<DependencyVulnerability>,
    pub scope: DependencyScope,
}

/// A job in a snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotJob {
    pub id: String,
    pub correlator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
}

/// A detector in a snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotDetector {
    pub name: String,
    pub version: String,
    pub url: Url,
}

/// File information for a snapshot manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotManifestFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
}

/// A resolved package dependency in a snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotResolvedPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<DependencyRelationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<DependencyScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
}

/// A manifest in a snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotManifest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<SnapshotManifestFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<HashMap<String, SnapshotResolvedPackage>>,
}

/// A snapshot of dependencies for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub version: u64,
    pub job: SnapshotJob,
    pub sha: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub detector: SnapshotDetector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifests: Option<HashMap<String, SnapshotManifest>>,
    pub scanned: DateTime<Utc>,
}

/// Response returned when creating a dependency snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateSnapshotResponse {
    pub id: u64,
    pub created_at: String,
    pub result: String,
    pub message: String,
}

/// A Software Bill of Materials (SBOM) wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[deprecated(
    note = "This operation is closing down and will not be accessible after November 13, 2026. Please migrate to the asynchronous flow using generate_report and fetch_report."
)]
pub struct Sbom {
    pub sbom: SbomDependencyGraph,
}
