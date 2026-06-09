use super::*;

#[derive(serde::Serialize)]
pub struct CreateIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::IssueHandler<'octo>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
    #[serde(skip)]
    sub_issues: Vec<CreateSubIssueBuilder<'octo, 'r>>,
}

impl<'octo, 'r> CreateIssueBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r super::IssueHandler<'octo>, title: String) -> Self {
        Self {
            handler,
            title,
            body: None,
            milestone: None,
            labels: None,
            assignees: None,
            sub_issues: Vec::new(),
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<models::issues::Issue> {
        let route = format!("/{}/issues", self.handler.repo);

        if self.sub_issues.is_empty() {
            return self.handler.crab.post(route, Some(&self)).await;
        }
        let parent: models::issues::Issue = self.handler.crab.post(route, Some(&self)).await?;
        for issue in &self.sub_issues {
            let sub_issue = issue.create().await?;
            issue.link(sub_issue.id, parent.number).await?;
        }
        Ok(parent)
    }

    /// The contents of the issue.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.body = body.into().map(A::into);
        self
    }

    /// The number of the milestone to associate this issue with. *NOTE: Only
    /// users with push access can set the milestone for new issues. The
    /// milestone is silently dropped otherwise.*
    pub fn milestone(mut self, milestone: impl Into<Option<u64>>) -> Self {
        self.milestone = milestone.into();
        self
    }

    /// Labels to associate with this issue. *NOTE: Only users with push access
    /// can set labels for new issues. Labels are silently dropped otherwise.*
    pub fn labels(mut self, labels: impl Into<Option<Vec<String>>>) -> Self {
        self.labels = labels.into();
        self
    }

    /// Logins for Users to assign to this issue. *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn assignees(mut self, assignees: impl Into<Option<Vec<String>>>) -> Self {
        self.assignees = assignees.into();
        self
    }

    /// Adds a new sub-issue under this issue *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn add_sub_issue(
        mut self,
        title: impl Into<String>,
        f: impl FnOnce(CreateSubIssueBuilder<'octo, 'r>) -> CreateSubIssueBuilder<'octo, 'r>,
    ) -> CreateIssueBuilder<'octo, 'r> {
        self.sub_issues
            .push(f(CreateSubIssueBuilder::new(self.handler, title.into())));
        self
    }
}
#[derive(serde::Serialize)]
pub struct CreateSubIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::IssueHandler<'octo>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
    #[serde(skip)]
    replace_parent: Option<bool>,
    #[serde(skip)]
    sub_issues: Vec<CreateSubIssueBuilder<'octo, 'r>>,
}

impl<'octo, 'r> CreateSubIssueBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r super::IssueHandler<'octo>, title: String) -> Self {
        Self {
            handler,
            title,
            body: None,
            milestone: None,
            labels: None,
            assignees: None,
            replace_parent: None,
            sub_issues: Vec::new(),
        }
    }

    async fn create(&self) -> crate::Result<models::issues::Issue> {
        let route = format!("/{}/issues", self.handler.repo);

        if self.sub_issues.is_empty() {
            return self.handler.crab.post(route, Some(&self)).await;
        }
        let parent: models::issues::Issue = self.handler.crab.post(route, Some(&self)).await?;
        for issue in &self.sub_issues {
            let sub_issue = Box::pin(issue.create()).await?;
            issue.link(sub_issue.id, parent.number).await?;
        }
        Ok(parent)
    }

    async fn link(&self, id: IssueId, parent_number: u64) -> crate::Result<models::issues::Issue> {
        let route = format!("/{}/issues/{}/sub_issues", self.handler.repo, parent_number);

        self.handler
            .crab
            .post(
                route,
                Some(&serde_json::json!({
                    "sub_issue_id": id,
                    "replace_parent": self.replace_parent.unwrap_or(false),
                })),
            )
            .await
    }

    /// The contents of the issue.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.body = body.into().map(A::into);
        self
    }

    /// The number of the milestone to associate this issue with. *NOTE: Only
    /// users with push access can set the milestone for new issues. The
    /// milestone is silently dropped otherwise.*
    pub fn milestone(mut self, milestone: impl Into<Option<u64>>) -> Self {
        self.milestone = milestone.into();
        self
    }

    /// Labels to associate with this issue. *NOTE: Only users with push access
    /// can set labels for new issues. Labels are silently dropped otherwise.*
    pub fn labels(mut self, labels: impl Into<Option<Vec<String>>>) -> Self {
        self.labels = labels.into();
        self
    }

    /// Logins for Users to assign to this issue. *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn assignees(mut self, assignees: impl Into<Option<Vec<String>>>) -> Self {
        self.assignees = assignees.into();
        self
    }

    /// Whether to replace the parent relationship for all sub-issue links. *NOTE: Only users
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn replace_parent(mut self, replace_parent: impl Into<Option<bool>>) -> Self {
        self.replace_parent = replace_parent.into();
        self
    }

    /// Adds a new sub-issue under this issue *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn add_sub_issue(
        mut self,
        title: impl Into<String>,
        f: impl FnOnce(CreateSubIssueBuilder<'octo, 'r>) -> CreateSubIssueBuilder<'octo, 'r>,
    ) -> CreateSubIssueBuilder<'octo, 'r> {
        self.sub_issues
            .push(f(CreateSubIssueBuilder::new(self.handler, title.into())));
        self
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("owner", "repo");
        let list = handler
            .create("test-issue")
            .body(String::from("testing..."))
            .milestone(3456)
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
