//! Get data about the currently authenticated user.

use crate::{models, Octocrab, Result};

/// Handler for the current authenication API. **Note** All of the methods
/// provided below require at least some authenication such as personal token
/// in order to be used.
///
/// Created with [`Octocrab::current`].
#[derive(octocrab_derive::Builder)]
pub struct CurrentAuthHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CurrentAuthHandler<'octo> {
    /// Fetches information about the current user.
    pub async fn user(&self) -> Result<models::User> {
        self.crab.get("user", None::<&()>).await
    }
}
