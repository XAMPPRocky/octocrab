use super::*;

#[octocrab_derive::serde_skip_none]
#[derive(serde::Serialize, octocrab_derive::Builder)]
pub struct UpdateIssueBuilder<'octo, 'handler, 'title, 'body, 'assignee, 'labels> {
    #[serde(skip)]
    handler: &'handler IssueHandler<'octo>,
    #[serde(skip)]
    number: u64,
    /// The title of the issue.
    title: Option<&'title str>,
    /// The body of the issue.
    body: Option<&'body str>,
    /// The assignees of the issue.
    assignees: Option<&'assignee [String]>,
    /// The state of the issue.
    state: Option<models::IssueState>,
    /// The milestone of the issue.
    milestone: Option<u64>,
    /// The labels of the issue.
    labels: Option<&'labels [String]>,
}

impl<'octo, 'handler, 'title, 'body, 'assignee, 'labels>
    UpdateIssueBuilder<'octo, 'handler, 'title, 'body, 'assignee, 'labels>
{
    /// Send the actual request.
    pub async fn send(self) -> Result<models::issues::Issue> {
        let route = format!(
            "repos/{owner}/{repo}/issues/{issue}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            issue = self.number,
        );

        self.handler.crab.patch(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("rust-lang", "rust");
        let assignees = &[String::from("ferris")];
        let labels = &[
            String::from("help wanted"),
            String::from("good first issue"),
        ];
        let update = handler
            .update(1234)
            .title("Updated title")
            .body("New body")
            .state(crate::models::IssueState::Closed)
            .milestone(1234u64)
            .assignees(assignees)
            .labels(labels);

        assert_eq!(
            serde_json::to_value(update).unwrap(),
            serde_json::json!({
                "title": "Updated title",
                "body": "New body",
                "state": "closed",
                "milestone": 1234,
                "assignees": ["ferris"],
                "labels": ["help wanted", "good first issue"],
            })
        )
    }
}
