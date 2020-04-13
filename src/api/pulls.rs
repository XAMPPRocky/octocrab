use crate::Octocrab;

pub struct PullRequestHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> PullRequestHandler<'octo> {
    pub fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    pub fn list(&self) -> ListPullRequestsBuilder {
        ListPullRequestsBuilder::new(self)
    }
}

pub struct ListPullRequestsBuilder<'octo, 'b> {
    handler: &'b PullRequestHandler<'octo>,
    state: Option<PullRequestState>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    All,
    Open,
    Closed
}

impl<'octo, 'b> ListPullRequestsBuilder<'octo, 'b> {
    fn new(handler: &'b PullRequestHandler<'octo>) -> Self {
        Self { handler, state: None, }
    }

    /// Filter pull requests by `PullRequestState`.
    pub fn state(mut self, state: PullRequestState) -> Self {
        self.state = Some(state);
        self
    }

    pub async fn send(self) -> Result<Vec<crate::models::PullRequest>, Box<dyn std::error::Error>> {
        let url = format!("/repos/{owner}/{repo}/pulls", owner = self.handler.owner, repo = self.handler.repo);
        let mut params = Vec::new();

        if let Some(state) = self.state {
            params.push(("state", state));
        }

        self.handler.crab.get(url, Some(&params)).await
    }
}
