//! The gist API

use snafu::ResultExt;
use http::StatusCode;

use crate::Octocrab;
use crate::params::gists::File;

/// Handler for GitHub's gist API.
///
/// Created with [`Octocrab::gists`].
pub struct GistHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> GistHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Check if a gist has been starred by the currently authenticated user.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// assert!(octocrab::instance().gists().check_is_starred("id").await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_is_starred(&self, id: &str) -> crate::Result<bool> {
        let url = self.crab.absolute_url(format!("gists/{}/star", id))?;

        let resp = self.crab._get(url, None::<&()>).await?;

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => {
                crate::map_github_error(resp).await?;
                unreachable!()
            }
        }
    }

    /// Create a new gist.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::gists::File;
    /// let gitignore = octocrab::instance()
    ///     .gists()
    ///     .create(&[
    ///         File::new("hello_world.rs", "fn main() {\n println!(\"Hello World!\");\n}")
    ///     ])
    ///     // Optional Parameters
    ///     .description("Hello World in Rust")
    ///     .public(false)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<'files>(&self, files: &'files [File]) -> CreateGistBuilder<'octo, 'files> {
        CreateGistBuilder::new(files)
    }


    /// Star a gist.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// assert!(octocrab::instance().gists().star("id").await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn star(&self, id: &str) -> crate::Result<bool> {
        let url = self.crab.absolute_url(format!("gists/{}/star", id))?;

        let resp = self.crab._put(url, None::<&()>).await?;

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => {
                crate::map_github_error(resp).await?;
                unreachable!()
            }
        }
    }

    /// Get a single gist.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gitignore = octocrab::instance().gitignore().get("C").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, name: impl AsRef<str>) -> crate::Result<String> {
        let route = format!("gitignore/templates/{name}", name = name.as_ref());
        let request = self
            .crab
            .client
            .get(self.crab.absolute_url(route)?)
            .header(reqwest::header::ACCEPT, crate::format_media_type("raw"));

        self.crab
            .execute(request)
            .await?
            .text()
            .await
            .context(crate::error::Http)
    }
}

