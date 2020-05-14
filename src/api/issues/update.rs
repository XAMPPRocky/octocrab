use super::*;
use crate::models;

#[derive(serde::Serialize)]
pub struct UpdateIssueBuilder<'octo, 'a, 'b, 'c, 'd, 'e> {
    #[serde(skip)]
    handler: &'a IssueHandler<'octo>,
    #[serde(skip)]
    number: u64,
    title: Option<&'b str>,
    body: Option<&'c str>,
    assignees: Option<&'d [String]>,
    state: Option<models::IssueState>,
    milestone: Option<u64>,
    labels: Option<&'e [String]>,
}

impl<'octo, 'a, 'b, 'c, 'd, 'e> UpdateIssueBuilder<'octo, 'a, 'b, 'c, 'd, 'e> {
    pub(crate) fn new(handler: &'a IssueHandler<'octo>, number: u64) -> Self {
        Self {
            handler,
            number,
            title: None,
            body: None,
            assignees: None,
            state: None,
            milestone: None,
            labels: None,
        }
    }

    /// The title of the issue.
    pub fn title(mut self, title: &'b (impl AsRef<str> + ?Sized)) -> Self {
        self.title = Some(title.as_ref());
        self
    }

    /// The body of the issue.
    pub fn body(mut self, body: &'c (impl AsRef<str> + ?Sized)) -> Self {
        self.body = Some(body.as_ref());
        self
    }

    /// The assignees of the issue.
    pub fn assignees(mut self, assignees: &'d (impl AsRef<[String]> + ?Sized)) -> Self {
        self.assignees = Some(assignees.as_ref());
        self
    }

    /// The state of the issue.
    pub fn state(mut self, state: impl Into<models::IssueState>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// The milestone of the issue.
    pub fn milestone(mut self, milestone: impl Into<u64>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    /// The labels of the issue.
    pub fn labels(mut self, labels: &'e (impl AsRef<[String]> + ?Sized)) -> Self {
        self.labels = Some(labels.as_ref());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<models::Issue> {
        let route = format!(
            "/repos/{owner}/{repo}/issues/{issue}",
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
