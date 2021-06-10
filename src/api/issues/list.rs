use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct ListIssuesBuilder<'octo, 'handler, 'assignee, 'labels> {
    #[serde(skip)]
    handler: &'handler IssueHandler<'octo>,
    /// Filter pull requests by `state`.
    state: Option<params::State>,
    /// If an integer is passed, it should refer to a milestone by its number field. If the string
    /// `"*"` is passed, issues with any milestone are accepted. If the string none is passed,
    /// issues without milestones are returned.
    milestone: Option<params::issues::Filter<u64>>,
    /// Filter by assignee, can be the name of a user. Pass in the string `"none"` for issues with
    /// no assigned user, and `"*"` for issues assigned to any user.
    assignee: Option<params::issues::Filter<&'assignee str>>,
    /// Filter by the creator of the issue.
    creator: Option<String>,
    /// Filter by the creator of the issue.
    mentioned: Option<String>,
    /// Filter issues by label.
    #[serde(serialize_with = "comma_separated")]
    labels: Option<&'labels [String]>,
    /// What to sort results by. Can be either `created`, `updated`, `popularity` (comment count) or
    /// `long-running` (age, filtering by pulls updated in the last month).
    sort: Option<params::issues::Sort>,
    /// The direction of the sort. Can be either ascending or descending. Default: descending when
    /// sort is `created` or sort is not specified, otherwise ascending sort.
    direction: Option<params::Direction>,
    /// Results per page (max 100).
    per_page: Option<u8>,
    /// Page number of the results to fetch.
    page: Option<u32>,
}

impl<'octo, 'handler, 'assignee, 'labels> ListIssuesBuilder<'octo, 'handler, 'assignee, 'labels> {
    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<models::issues::Issue>> {
        let url = format!(
            "repos/{owner}/{repo}/issues",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

fn comma_separated<S: serde::Serializer>(
    labels: &Option<&[String]>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&labels.unwrap().join(","))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("rust-lang", "rust");
        let labels = vec![
            String::from("help wanted"),
            String::from("good first issue"),
        ];
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .milestone(1234)
            .assignee("ferris")
            .creator("octocrab")
            .mentioned("octocat")
            .labels(&labels)
            .sort(crate::params::issues::Sort::Comments)
            .direction(crate::params::Direction::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "milestone": 1234,
                "assignee": "ferris",
                "creator": "octocrab",
                "mentioned": "octocat",
                "labels": "help wanted,good first issue",
                "sort": "comments",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
