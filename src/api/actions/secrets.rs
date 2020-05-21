use super::*;

#[derive(serde::Serialize)]
pub struct CreateOrUpdateOrgSecretBuilder<'octo, 'r, 'text> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    org: String,
    secret_name: String,
    mode: Option<crate::params::markdown::Mode>,
    context: Option<String>,
}

impl<'octo, 'r, 'text> CreateOrUpdateOrgSecretBuilder<'octo, 'r, 'text> {
    pub(crate) fn new(handler: &'r ActionsHandler<'octo>, org: String, secret_name: String) -> Self {
        Self {
            handler,
            org,
            secret_name,
        }
    }

    /// The repository context to use when creating references in `Mode::Gfm`.
    /// Omit this parameter when using markdown mode.
    pub fn context<A: Into<String>>(mut self, context: impl Into<Option<A>>) -> Self {
        self.context = context.into().map(A::into);
        self
    }

    /// The rendering mode.
    pub fn mode(mut self, mode: impl Into<Option<crate::params::markdown::Mode>>) -> Self {
        self.mode = mode.into();
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<String> {
        self.handler
            .crab
            ._post(self.handler.crab.absolute_url("/markdown")?, Some(&self))
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

