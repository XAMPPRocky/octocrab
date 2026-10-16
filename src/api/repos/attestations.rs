use crate::api::repos::RepoHandler;
use crate::models::{AttestationsResponse, CreateAttestationResponse};

/// Handler for GitHub's repository attestations API.
///
/// Created with [`RepoHandler::attestations`].
pub struct RepoAttestationsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoAttestationsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Create an attestation for a repository.
    ///
    /// `POST /repos/{owner}/{repo}/attestations`
    pub async fn create(
        &self,
        bundle: serde_json::Value,
    ) -> crate::Result<CreateAttestationResponse> {
        let route = format!("/{}/attestations", self.handler.repo);
        let body = serde_json::json!({ "bundle": bundle });
        self.handler.crab.post(route, Some(&body)).await
    }

    /// List attestations for a subject digest for a repository.
    ///
    /// `GET /repos/{owner}/{repo}/attestations/{subject_digest}`
    pub fn list(
        &self,
        subject_digest: impl Into<String>,
    ) -> ListRepoAttestationsBuilder<'octo, 'r> {
        ListRepoAttestationsBuilder::new(self.handler, subject_digest.into())
    }
}

/// Builder for [`RepoAttestationsHandler::list`].
#[derive(serde::Serialize)]
pub struct ListRepoAttestationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    subject_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predicate_type: Option<String>,
}

impl<'octo, 'r> ListRepoAttestationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, subject_digest: String) -> Self {
        Self {
            handler,
            subject_digest,
            per_page: None,
            before: None,
            after: None,
            predicate_type: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// A cursor to fetch results before.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// A cursor to fetch results after.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Optional filter for fetching attestations with a given predicate type.
    pub fn predicate_type(mut self, predicate_type: impl Into<String>) -> Self {
        self.predicate_type = Some(predicate_type.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<AttestationsResponse> {
        let route = format!(
            "/{}/attestations/{}",
            self.handler.repo, self.subject_digest
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
