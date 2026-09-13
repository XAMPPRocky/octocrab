use http::StatusCode;

use crate::api::repos::RepoRef;
use crate::from_response::FromResponse;
use crate::models::repos::sbom::{SbomDependencyGraph, SbomFetchResponse};
use crate::Octocrab;

/// A client to GitHub's SBOM APIs.
///
/// Created with [`crate::api::repos::RepoHandler::sbom`]
pub struct RepoSbomHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> RepoSbomHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    /// Exports the software bill of materials (SBOM) for a repository in SPDX JSON format.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/dependency-graph/sboms?apiVersion=2022-11-28#export-a-software-bill-of-materials-sbom-for-a-repository)
    #[deprecated(
        note = "This operation is closing down and will not be accessible after November 13, 2026. Please migrate to the asynchronous flow using generate_report and fetch_report."
    )]
    #[allow(deprecated)]
    pub async fn export(&self) -> crate::Result<crate::models::dependency_graph::Sbom> {
        let route = format!("/{}/dependency-graph/sbom", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Shortcut for [`Self::export`].
    #[deprecated(
        note = "This operation is closing down and will not be accessible after November 13, 2026. Please migrate to the asynchronous flow using generate_report and fetch_report."
    )]
    #[allow(deprecated)]
    pub async fn get(&self) -> crate::Result<crate::models::dependency_graph::Sbom> {
        self.export().await
    }

    /// Trigger the report generation, and get the URL for accessing that
    /// report.
    ///
    /// You need permission to read the contents of the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let sbom_req = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .sbom()
    ///     .generate_report()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_report(
        &self,
    ) -> crate::Result<crate::models::repos::sbom::SbomGenerateReportResponse> {
        // Fetch the SBOM URL
        let route = format!("/{}/dependency-graph/sbom/generate-report", self.repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Fetch the generated report, or report back that it's not ready.
    /// - `::Ready` indicates that the report is ready.
    /// - `::NotReady` indicates that the report isn't ready yet, try again later.
    /// - `::NotFound` indicates the report wasn't found, and should be re-generated.
    /// - `::Broken` is a wheels-fall-off condition indicating that though the server returned a report, it could not be serialized.
    ///
    /// You need permission to read the contents of the repository.
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn run() -> octocrab::Result<()> {
    /// #   let sbom_req = octocrab::instance()
    /// #       .repos("owner", "repository")
    /// #       .sbom()
    /// #       .generate_report()
    /// #       .await?;
    /// #
    /// # use octocrab::models::repos::sbom::{SbomDependencyGraph, SbomFetchResponse};
    /// # use tokio::time::{Duration, sleep};
    /// let the_graph: Box<SbomDependencyGraph>;
    /// loop {
    ///     let sbom_graph = octocrab::instance()
    ///         .repos("owner", "repository")
    ///         .sbom()
    ///         .fetch_report(sbom_req.clone())
    ///         .await?;
    ///
    ///     match sbom_graph {
    ///         SbomFetchResponse::Ready { graph } => {
    ///             the_graph = graph;
    ///             break;
    ///         }
    ///         _ => {
    ///             sleep(Duration::from_millis(500)).await;
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_report(
        &self,
        report_url: crate::models::repos::sbom::SbomGenerateReportResponse,
    ) -> crate::Result<crate::models::repos::sbom::SbomFetchResponse> {
        // Find the UUID in the URL
        if let Some(report_uuid) = report_url
            .sbom_url
            .path_segments()
            .and_then(|mut s| s.next_back())
        {
            // Make sure that it's possible for this to be a UUID
            if !report_uuid
                .chars()
                .all(|c| c.is_ascii_hexdigit() || c == '-')
            {
                return Err(crate::Error::Other {
                    source: format!("Invalid characters in UUID value '{}'", report_uuid).into(),
                    backtrace: snafu::Backtrace::capture(),
                });
            }

            // Make the call
            let response = self
                .crab
                ._get(format!(
                    "/{}/dependency-graph/sbom/fetch-report/{}",
                    self.repo, report_uuid
                ))
                .await?;

            // Handle all of the cases...
            match response.status() {
                StatusCode::NOT_FOUND => Ok(SbomFetchResponse::NotFound),
                StatusCode::ACCEPTED => Ok(SbomFetchResponse::NotReady),
                // The successful call is a 302 which the underlying library follows for us.
                // We need only treat it as a 200.
                StatusCode::OK => {
                    if let Ok(target_graph) = <SbomDependencyGraph>::from_response(response).await {
                        Ok(SbomFetchResponse::Ready {
                            graph: Box::new(target_graph),
                        })
                    } else {
                        Ok(SbomFetchResponse::Broken)
                    }
                }
                _ => Err(crate::map_github_error(response).await.unwrap_err()),
            }
        } else {
            Err(crate::Error::Other {
                source: format!(
                    "Unable to find report ID in URL '{}'.",
                    report_url.sbom_url.as_str()
                )
                .into(),
                backtrace: snafu::Backtrace::capture(),
            })
        }
    }
}
