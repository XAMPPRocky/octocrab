use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListReposBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b OrgHandler<'octo>,
    /// Filter repostories by their type.
    #[builder(rename = "repo_type")]
    #[serde(rename = "type")]
    ty: Option<crate::params::repos::Type>,
    /// What to sort results by. Can be either `created`, `updated`, `pushed`, or `full_name`.
    sort: Option<crate::params::repos::Sort>,
    /// The direction of the sort. Can be either `Ascending` or `Descending`. Default: ascending
    /// when sort is `full_name`, otherwise descending.
    direction: Option<crate::params::Direction>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'b> ListReposBuilder<'octo, 'b> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Repository>> {
        let url = format!("orgs/{owner}/repos", owner = self.handler.owner);
        self.handler.crab.get(url, Some(&self)).await
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
