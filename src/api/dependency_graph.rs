//! GitHub Dependency Graph API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph?apiVersion=2022-11-28)

pub use crate::api::repos::dependency_graph::{
    CompareDependenciesBuilder, RepoDependencyGraphHandler,
};
