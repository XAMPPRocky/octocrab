use super::*;

/// A builder pattern struct for listing pull requests.
///
/// created by [`PullRequestHandler::list`]
///
/// [`PullRequestHandler::list`]: ./struct.PullRequestHandler.html#method.list
#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListPullRequestsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    /// Filter pull requests by `state`.
    state: Option<crate::params::State>,
    /// Filter pull requests by head user or head organization and branch name in the format of
    /// `user:ref-name` or `organization:ref-name`. For example: `github:new-script-format` or
    /// `octocrab:test-branch`.
    head: Option<String>,
    /// Filter pulls by base branch name. Example: `gh-pages`.
    base: Option<String>,
    /// What to sort results by. Can be either `created`, `updated`, `popularity` (comment count) or
    /// `long-running` (age, filtering by pulls updated in the last month).
    sort: Option<crate::params::pulls::Sort>,
    /// The direction of the sort. Can be either ascending or descending. Default: descending when
    /// sort is `created` or sort is not specified, otherwise ascending sort.
    direction: Option<crate::params::Direction>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'b> ListPullRequestsBuilder<'octo, 'b> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::pulls::PullRequest>> {
        let url = format!(
            "repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
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
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .head("master")
            .base("branch")
            .sort(crate::params::pulls::Sort::Popularity)
            .direction(crate::params::Direction::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "head": "master",
                "base": "branch",
                "sort": "popularity",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
