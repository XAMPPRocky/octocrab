use crate::{models, Octocrab, Result};

pub struct CurrentAuthHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CurrentAuthHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Fetches information about the current user.
    pub async fn user(&self) -> Result<models::User> {
        self.crab.get("/user", None::<&()>).await
    }
}
