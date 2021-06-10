use super::*;

/// A builder pattern struct for listing comments.
///
/// created by [`PullRequestHandler::list_comments`]
///
/// [`PullRequestHandler::list_comments`]: ./struct.PullRequestHandler.html#method.list_comments
#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListCommentsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[builder(skip)]
    #[serde(skip)]
    pr: Option<u64>,
    /// What to sort results by. Can be either `created` or `updated`.
    sort: Option<crate::params::pulls::comments::Sort>,
    /// The direction of the sort. Can be either ascending or descending.  Default: descending when
    /// sort is `created` or sort is not specified, otherwise ascending sort.
    direction: Option<crate::params::Direction>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
    /// Only show notifications updated after the given time.
    since: Option<chrono::DateTime<chrono::Utc>>,
}

impl<'octo, 'b> ListCommentsBuilder<'octo, 'b> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::pulls::Comment>> {
        let url = format!(
            "repos/{owner}/{repo}/pulls/{pr}comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = if let Some(pr) = self.pr {
                pr.to_string() + "/"
            } else {
                "".into()
            },
        );
        self.handler.http_get(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let yesterday = chrono::Utc::now() - chrono::Duration::days(1);
        let list = handler
            .list_comments(Some(1))
            .sort(crate::params::pulls::comments::Sort::Updated)
            .direction(crate::params::Direction::Ascending)
            .since(yesterday)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "sort": "updated",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
                "since": yesterday
            })
        )
    }
}
