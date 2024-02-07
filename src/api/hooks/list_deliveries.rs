use super::*;

/// A builder pattern struct for listing hooks deliveries.
///
/// created by [`HooksHandler::list_deliveries`]
///
/// [`HooksHandler::list_deliveries`]: ./struct.HooksHandler.html#method.list_deliveries
#[derive(serde::Serialize)]
pub struct ListHooksDeliveriesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r HooksHandler<'octo>,
    #[serde(skip)]
    hook_id: HookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}
impl<'octo, 'r> ListHooksDeliveriesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r HooksHandler<'octo>, hook_id: HookId) -> Self {
        Self {
            handler,
            hook_id,
            per_page: None,
            cursor: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::hooks::Delivery>> {
        let route= match self.handler.repo.clone() {
            Some(repo) => format!("/repos/{}/{}/hooks/{}/deliveries", self.handler.owner, repo, self.hook_id),
            None => format!("/orgs/{}/hooks/{}/deliveries", self.handler.owner, self.hook_id),
        };
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    // use crate::models::HookId;

    // #[tokio::test]
    // async fn serialize() {
    //     let octocrab = crate::Octocrab::default();
    //     let handler = octocrab.hooks("rust-lang");
    //     let list = handler
    //         .list_deliveries(HookId::from(21u64))
    //         .per_page(100)
    //         .page(1u8);
    //
    //     assert_eq!(
    //         serde_json::to_value(list).unwrap(),
    //         serde_json::json!({
    //             "state": "open",
    //             "head": "master",
    //             "base": "branch",
    //             "sort": "popularity",
    //             "direction": "asc",
    //             "per_page": 100,
    //             "page": 1,
    //         })
    //     )
    // }
}
