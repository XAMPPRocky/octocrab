//! The metadata API.

use http::request;

use crate::models::meta::{ApiRoot, Meta};
use crate::Octocrab;

/// Handler for GitHub's Meta API.
///
/// Created with [`Octocrab::meta`].
pub struct MetaHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> MetaHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Get Hypermedia links to resources accessible in GitHub's REST API.
    ///
    /// See also: [GitHub API Root](https://docs.github.com/en/rest/meta/meta?apiVersion=2022-11-28#github-api-root)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let root = octocrab::instance().meta().get_api_root().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_api_root(&self) -> crate::Result<ApiRoot> {
        self.crab.get("/", None::<&()>).await
    }

    /// Returns meta information about GitHub, including a list of GitHub's IP addresses.
    ///
    /// See also: [Get GitHub meta information](https://docs.github.com/en/rest/meta/meta?apiVersion=2022-11-28#get-github-meta-information)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let info = octocrab::instance().meta().get_github_meta_information().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_github_meta_information(&self) -> crate::Result<Meta> {
        self.crab.get("/meta", None::<&()>).await
    }

    /// Get the octocat as ASCII art.
    ///
    /// # Arguments
    ///
    /// * `s` - The words to show in Octocat's speech bubble.
    ///
    /// See also: [Get Octocat](https://docs.github.com/en/rest/meta/meta?apiVersion=2022-11-28#get-octocat)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let art = octocrab::instance().meta().get_octocat(Some("hello")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_octocat(&self, s: Option<impl AsRef<str>>) -> crate::Result<String> {
        let route = "/octocat";
        #[derive(serde::Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            s: Option<&'a str>,
        }
        let query = Query {
            s: s.as_ref().map(|s| s.as_ref()),
        };
        let uri = self.crab.parameterized_uri(route, Some(&query))?;
        let mut request = request::Builder::new().method("GET").uri(uri);
        request = request.header(http::header::ACCEPT, "application/octocat-stream");

        let request = self.crab.build_request(request, None::<&()>)?;
        let response = self.crab.execute(request).await?;
        self.crab.body_to_string(response).await
    }

    /// Get all supported GitHub API versions.
    ///
    /// See also: [Get all API versions](https://docs.github.com/en/rest/meta/meta?apiVersion=2022-11-28#get-all-api-versions)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let versions = octocrab::instance().meta().get_api_versions().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_api_versions(&self) -> crate::Result<Vec<String>> {
        self.crab.get("/versions", None::<&()>).await
    }

    /// Get a random sentence from the Zen of GitHub.
    ///
    /// See also: [Get the Zen of GitHub](https://docs.github.com/en/rest/meta/meta?apiVersion=2022-11-28#get-the-zen-of-github)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let sentence = octocrab::instance().meta().zen().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn zen(&self) -> crate::Result<String> {
        let uri = self.crab.parameterized_uri("/zen", None::<&()>)?;
        let mut request = request::Builder::new().method("GET").uri(uri);
        request = request.header(http::header::ACCEPT, "text/plain");

        let request = self.crab.build_request(request, None::<&()>)?;
        let response = self.crab.execute(request).await?;
        self.crab.body_to_string(response).await
    }
}
