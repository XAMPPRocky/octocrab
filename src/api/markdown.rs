//! The markdown API
use snafu::ResultExt;

use crate::Octocrab;

/// Handler for GitHub's markdown API.
///
/// Created with [`Octocrab::markdown`].
#[derive(octocrab_derive::Builder)]
pub struct MarkdownHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> MarkdownHandler<'octo> {
    /// Render an arbitrary Markdown document.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let markdown = octocrab::instance()
    ///     .markdown()
    ///     .render("Comment referencing issue #404")
    ///     .mode(params::markdown::Mode::Gfm)
    ///     .context("owner/repo")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn render<'r, 'text>(
        &'r self,
        text: &'text (impl AsRef<str> + ?Sized),
    ) -> RenderMarkdownBuilder<'octo, 'r, 'text> {
        RenderMarkdownBuilder::new(self, text.as_ref())
    }

    /// Render a Markdown document in raw mode.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let markdown = octocrab::instance()
    ///     .markdown()
    ///     .render_raw("~~_**Octocrab**_~~")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn render_raw(&self, text: impl Into<String>) -> crate::Result<String> {
        let request = self
            .crab
            .client
            .post(self.crab.absolute_url("markdown/raw")?)
            .header(reqwest::header::CONTENT_TYPE, "text/x-markdown")
            .body(text.into());

        self.crab
            .execute(request)
            .await?
            .text()
            .await
            .context(crate::error::Http)
    }
}

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct RenderMarkdownBuilder<'octo, 'r, 'text> {
    #[serde(skip)]
    handler: &'r MarkdownHandler<'octo>,
    text: &'text str,
    /// The rendering mode.
    mode: Option<crate::params::markdown::Mode>,
    /// The repository context to use when creating references in `Mode::Gfm`. Omit this parameter
    /// when using markdown mode.
    context: Option<String>,
}

impl<'octo, 'r, 'text> RenderMarkdownBuilder<'octo, 'r, 'text> {
    /// Send the actual request.
    pub async fn send(self) -> crate::Result<String> {
        self.handler
            .crab
            ._post(self.handler.crab.absolute_url("markdown")?, Some(&self))
            .await?
            .text()
            .await
            .context(crate::error::Http)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn serialize() {
        let octocrab = crate::instance();
        let handler = octocrab.markdown();
        let render = handler
            .render("**Markdown**")
            .mode(crate::params::markdown::Mode::Gfm)
            .context("owner/repo");

        assert_eq!(
            serde_json::to_value(render).unwrap(),
            serde_json::json!({
                "text": "**Markdown**",
                "mode": "gfm",
                "context": "owner/repo",
            })
        )
    }
}
