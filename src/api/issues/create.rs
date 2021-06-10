use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct CreateIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::IssueHandler<'octo>,
    title: String,
    /// The contents of the issue.
    body: Option<String>,
    /// The number of the milestone to associate this issue with. *NOTE: Only  users with push
    /// access can set the milestone for new issues. The milestone is silently dropped otherwise.*
    milestone: Option<u64>,
    /// Labels to associate with this issue. *NOTE: Only users with push access can set labels for
    /// new issues. Labels are silently dropped otherwise.*
    labels: Option<Vec<String>>,
    /// Logins for Users to assign to this issue. *NOTE: Only users with push access can set
    /// assignees for new issues. Assignees are silently dropped otherwise.*
    assignees: Option<Vec<String>>,
}

impl<'octo, 'r> CreateIssueBuilder<'octo, 'r> {

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<models::issues::Issue> {
        let route = format!(
            "repos/{owner}/{repo}/issues",
            owner = self.handler.owner,
            repo = self.handler.repo,
        );

        self.handler.crab.post(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("owner", "repo");
        let list = handler
            .create("test-issue")
            .body(String::from("testing..."))
            .milestone(3456_u64)
            .labels(vec![String::from("help-wanted")])
            .assignees(vec![String::from("octocrab"), String::from("ferris")]);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "title": "test-issue",
                "body": "testing...",
                "milestone": 3456,
                "labels": ["help-wanted"],
                "assignees": ["octocrab", "ferris"],
            })
        )
    }
}
