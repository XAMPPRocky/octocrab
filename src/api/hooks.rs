//! The hooks API.
use crate::models::{HookDeliveryId, HookId};
use crate::Octocrab;

mod list_deliveries;
mod retry_delivery;

pub use self::{list_deliveries::ListHooksDeliveriesBuilder, retry_delivery::RetryDeliveryBuilder};

/// A client to GitHub's webhooks API.
///
/// Created with [`Octocrab::hooks`].
pub struct HooksHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: Option<String>,
}

impl<'octo> HooksHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String) -> Self {
        Self {
            crab,
            owner,
            repo: None,
        }
    }

    pub fn repo(mut self, repo: String) -> Self {
        self.repo = Some(repo);
        self
    }

    /// Lists all of the `Delivery`s associated with the hook.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let reviews = octocrab::instance()
    ///     .hooks("owner")
    ///     //.repo("repo")
    ///     .list_deliveries(21u64.into())
    ///     .per_page(100)
    ///     .page(2u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_deliveries(&self, hook_id: HookId) -> ListHooksDeliveriesBuilder<'_, '_> {
        ListHooksDeliveriesBuilder::new(self, hook_id)
    }

    /// Retry a delivery.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let reviews = octocrab::instance()
    ///     .hooks("owner")
    ///     //.repo("repo")
    ///     .retry_delivery(20u64.into(), 21u64.into())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn retry_delivery(
        &self,
        hook_id: HookId,
        delivery_id: HookDeliveryId,
    ) -> RetryDeliveryBuilder<'_, '_> {
        RetryDeliveryBuilder::new(self, hook_id, delivery_id)
    }

    /// Gets a delivery for a webhook.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28#get-a-delivery-for-a-repository-webhook)
    pub async fn get_delivery(
        &self,
        hook_id: HookId,
        delivery_id: HookDeliveryId,
    ) -> crate::Result<crate::models::hooks::DeliveryDetail> {
        let route = match self.repo.clone() {
            Some(repo) => format!(
                "/repos/{}/{}/hooks/{}/deliveries/{}",
                self.owner, repo, hook_id, delivery_id
            ),
            None => format!(
                "/orgs/{}/hooks/{}/deliveries/{}",
                self.owner, hook_id, delivery_id
            ),
        };
        self.crab.get(route, None::<&()>).await
    }
}
