//! The gitignore API

use snafu::ResultExt;

use crate::Octocrab;

/// Handler for GitHub's gitignore API.
///
/// Created with [`Octocrab::gitignore`].
pub struct GitignoreHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> GitignoreHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// List all templates available to pass as an option when creating a
    /// repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let list = octocrab::instance().gitignore().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> crate::Result<Vec<String>> {
        self.crab.get("gitignore/templates", None::<&()>).await
    }

    /// Get the source of a single template.
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
