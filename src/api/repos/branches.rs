use super::*;

#[derive(serde::Serialize)]
pub struct ListBranchesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListBranchesBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            protected: None,
            per_page: None,
            page: None,
        }
    }

    /// Setting to true returns only protected branches. When set to false, only
    /// unprotected branches are returned. Omitting this parameter returns all
    /// branches.
    pub fn protected(mut self, protected: impl Into<bool>) -> Self {
        self.protected = Some(protected.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<crate::Page<models::repos::Branch>> {
        let url = format!(
            "repos/{owner}/{repo}/branches",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

#[cfg(test)]
#[tokio::test]
async fn test() {
    let octocrab = crate::instance();
    let branches = octocrab
        .repos("rust-lang", "rust")
        .list_branches()
        .send()
        .await
        .unwrap();
    let master_branch = crate::models::repos::Branch {
        name: "master".to_string(),
        commit: crate::models::repos::CommitObject {
            sha: "35a0407814a6b5a04f0929105631e9c69e293e9d".to_string(),
            url: url::Url::parse("https://api.github.com/repos/rust-lang/rust/commits/35a0407814a6b5a04f0929105631e9c69e293e9d").unwrap(),
        },
        protected: true,
    };
    assert!(branches.items.contains(&master_branch));
}
