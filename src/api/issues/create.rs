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
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<models::issues::Issue> {
        let route = format!("/{}/issues", self.handler.repo);

        self.handler.crab.post(route, Some(&self)).await
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
        self,
        title: impl Into<String>,
    ) -> CreateIssueWithSubIssuesBuilder<'octo, 'r> {
        CreateIssueWithSubIssuesBuilder::new(
            self.handler,
            self.title,
            self.body,
            self.milestone,
            self.labels,
            self.assignees,
            title.into(),
        )
    }
}

#[derive(serde::Serialize)]
pub struct CreateIssueWithSubIssuesBuilder<'octo, 'r> {
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
    sub_issues: Vec<SubIssueSpec>,
    #[serde(skip)]
    current: SubIssueSpec,
}
impl<'octo, 'r> CreateIssueWithSubIssuesBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r super::IssueHandler<'octo>,
        title: String,
        body: Option<String>,
        milestone: Option<u64>,
        labels: Option<Vec<String>>,
        assignees: Option<Vec<String>>,
        first_sub_issue_title: String,
    ) -> Self {
        Self {
            handler,
            title,
            body,
            milestone,
            labels,
            assignees,
            sub_issues: Vec::new(),
            current: SubIssueSpec::new(first_sub_issue_title),
        }
    }

    /// Sends the actual request.
    pub async fn send(mut self) -> crate::Result<CreatedIssueWithSubIssues> {
        let create_issue_route = format!("/{}/issues", self.handler.repo);

        //create parent issue
        let parent: models::issues::Issue = self
            .handler
            .crab
            .post(create_issue_route.clone(), Some(&self))
            .await?;

        let add_sub_issue_route =
            format!("/{}/issues/{}/sub_issues", self.handler.repo, parent.number);
        self.sub_issues.push(self.current);

        let mut sub_issues = Vec::new();
        for spec in self.sub_issues {
            //create each sub-issue
            let sub_issue: models::issues::Issue = self
                .handler
                .crab
                .post(create_issue_route.clone(), Some(&spec))
                .await?;

            // link each sub-issue with the parent
            let _: models::issues::Issue = self
                .handler
                .crab
                .post(
                    add_sub_issue_route.clone(),
                    Some(&serde_json::json!({
                        "sub_issue_id": sub_issue.id,
                        "replace_parent": spec.replace_parent.unwrap_or(false),
                    })),
                )
                .await?;
            sub_issues.push(sub_issue);
        }

        Ok(CreatedIssueWithSubIssues { parent, sub_issues })
    }

    /// The contents of this sub-issue.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.current.body = body.into().map(A::into);
        self
    }

    /// The number of the milestone to associate this sub-issue with. *NOTE: Only
    /// users with push access can set the milestone for new issues. The
    /// milestone is silently dropped otherwise.*
    pub fn milestone(mut self, milestone: impl Into<Option<u64>>) -> Self {
        self.current.milestone = milestone.into();
        self
    }

    /// Labels to associate with this sub-issue. *NOTE: Only users with push access
    /// can set labels for new issues. Labels are silently dropped otherwise.*
    pub fn labels(mut self, labels: impl Into<Option<Vec<String>>>) -> Self {
        self.current.labels = labels.into();
        self
    }

    /// Logins for Users to assign to this sub-issue. *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn assignees(mut self, assignees: impl Into<Option<Vec<String>>>) -> Self {
        self.current.assignees = assignees.into();
        self
    }

    /// Whether to replace the parent relationship for all sub-issue links. *NOTE: Only users
    /// with push access can set assignees for new issues. Assignees are
    /// silently dropped otherwise.*
    pub fn replace_parent(mut self, replace_parent: impl Into<Option<bool>>) -> Self {
        self.current.replace_parent = replace_parent.into();
        self
    }

    /// Adds a new sub-issue under the parent of this issue *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn add_sub_issue(mut self, title: impl Into<String>) -> Self {
        self.sub_issues.push(self.current);
        self.current = SubIssueSpec::new(title.into());
        self
    }
}

pub struct CreatedIssueWithSubIssues {
    pub parent: models::issues::Issue,
    pub sub_issues: Vec<models::issues::Issue>,
}

#[derive(Debug, serde::Serialize)]
struct SubIssueSpec {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_parent: Option<bool>,
}
impl SubIssueSpec {
    fn new(title: String) -> Self {
        Self {
            title,
            body: None,
            milestone: None,
            labels: None,
            assignees: None,
            replace_parent: None,
        }
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
