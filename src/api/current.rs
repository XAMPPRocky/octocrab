//! Get data about the currently authenticated user.

use crate::{models, Octocrab, Result};

/// Handler for the current authenication API. **Note** All of the methods
/// provided below require at least some authenication such as personal token
/// in order to be used.
///
/// Created with [`Octocrab::current`].
pub struct CurrentAuthHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CurrentAuthHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Fetches information about the current user.
    pub async fn user(&self) -> Result<models::User> {
        self.crab.get("user", None::<&()>).await
    }

    /// Fetches information about the currently authenticated app.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///
    /// let app = octocrab
    ///     .current()
    ///     .app()
    ///     .await?;
    ///
    /// println!("{}", app.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn app(&self) -> Result<models::App> {
        self.crab.get("app", None::<&()>).await
    }
}
