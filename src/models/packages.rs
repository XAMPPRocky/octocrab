use serde::{Deserialize, Serialize};
use url::Url;

use super::{MinimalRepository, SimpleUser};
pub use super::{PackageId, PackageVersionId};

/// A package managed by GitHub Packages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Package {
    pub id: PackageId,
    pub name: String,
    pub package_type: PackageType,
    pub url: Url,
    pub html_url: Url,
    pub version_count: usize,
    pub visibility: PackageVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<SimpleUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<MinimalRepository>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// A specific version of a GitHub Package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PackageVersion {
    pub id: PackageVersionId,
    pub name: String,
    pub url: Url,
    pub package_html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PackageVersionMetadata>,
}

/// Metadata for a package version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PackageVersionMetadata {
    pub package_type: PackageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker: Option<DockerMetadata>,
}

/// Container-specific metadata for a package version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerMetadata {
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Docker-specific metadata for a package version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DockerMetadata {
    #[serde(default)]
    pub tag: Vec<String>,
}

/// The type of supported GitHub package.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PackageType {
    Npm,
    Maven,
    Rubygems,
    Docker,
    Nuget,
    Container,
}

impl std::fmt::Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageType::Npm => write!(f, "npm"),
            PackageType::Maven => write!(f, "maven"),
            PackageType::Rubygems => write!(f, "rubygems"),
            PackageType::Docker => write!(f, "docker"),
            PackageType::Nuget => write!(f, "nuget"),
            PackageType::Container => write!(f, "container"),
        }
    }
}

/// The visibility of a package.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PackageVisibility {
    Public,
    Private,
    Internal,
}

impl std::fmt::Display for PackageVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageVisibility::Public => write!(f, "public"),
            PackageVisibility::Private => write!(f, "private"),
            PackageVisibility::Internal => write!(f, "internal"),
        }
    }
}

/// The state of a package version (active or deleted).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PackageVersionState {
    Active,
    Deleted,
}

impl std::fmt::Display for PackageVersionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageVersionState::Active => write!(f, "active"),
            PackageVersionState::Deleted => write!(f, "deleted"),
        }
    }
}
