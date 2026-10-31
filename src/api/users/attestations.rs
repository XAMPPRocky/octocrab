use crate::api::users::UserHandler;
use crate::models::{AttestationsResponse, BulkAttestationsResponse};

/// Handler for GitHub's user attestations API.
///
/// Created with [`UserHandler::attestations`].
pub struct UserAttestationsHandler<'octo, 'r> {
    handler: &'r UserHandler<'octo>,
}

impl<'octo, 'r> UserAttestationsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r UserHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List attestations by bulk subject digests for a user.
    ///
    /// `POST /users/{username}/attestations/bulk-list`
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#list-attestations-by-bulk-subject-digests)
    pub fn bulk_list(
        &self,
        subject_digests: Vec<String>,
    ) -> BulkListUserAttestationsBuilder<'octo, 'r> {
        BulkListUserAttestationsBuilder::new(self.handler, subject_digests)
    }

    /// Request deletion of attestations by subject digests for a user.
    ///
    /// `POST /users/{username}/attestations/delete-request`
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#delete-attestations-in-bulk)
    pub async fn request_delete_by_digests(
        &self,
        subject_digests: Vec<String>,
    ) -> crate::Result<()> {
        let route = format!("/{}/attestations/delete-request", self.handler.user);
        let body = serde_json::json!({
            "subject_digests": subject_digests,
        });
        let response = self.handler.crab._post(route, Some(&body)).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Delete attestations by subject digest for a user.
    ///
    /// `DELETE /users/{username}/attestations/digest/{subject_digest}`
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#delete-attestations-by-subject-digest)
    pub async fn delete_by_digest(&self, subject_digest: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/{}/attestations/digest/{}",
            self.handler.user,
            subject_digest.as_ref()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Delete an attestation by ID for a user.
    ///
    /// `DELETE /users/{username}/attestations/{attestation_id}`
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#delete-attestations-by-id)
    pub async fn delete_by_id(&self, attestation_id: u64) -> crate::Result<()> {
        let route = format!("/{}/attestations/{attestation_id}", self.handler.user);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List attestations for a subject digest for a user.
    ///
    /// `GET /users/{username}/attestations/{subject_digest}`
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#list-attestations)
    pub fn list(
        &self,
        subject_digest: impl Into<String>,
    ) -> ListUserAttestationsBuilder<'octo, 'r> {
        ListUserAttestationsBuilder::new(self.handler, subject_digest.into())
    }
}

#[derive(serde::Serialize)]
struct BulkListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
}

#[derive(serde::Serialize)]
struct BulkListBody {
    subject_digests: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predicate_type: Option<String>,
}

/// Builder for [`UserAttestationsHandler::bulk_list`].
pub struct BulkListUserAttestationsBuilder<'octo, 'r> {
    handler: &'r UserHandler<'octo>,
    query: BulkListQuery,
    body: BulkListBody,
}

impl<'octo, 'r> BulkListUserAttestationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r UserHandler<'octo>, subject_digests: Vec<String>) -> Self {
        Self {
            handler,
            query: BulkListQuery {
                per_page: None,
                before: None,
                after: None,
            },
            body: BulkListBody {
                subject_digests,
                predicate_type: None,
            },
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.query.per_page = Some(per_page.into());
        self
    }

    /// A cursor to fetch results before.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.query.before = Some(before.into());
        self
    }

    /// A cursor to fetch results after.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.query.after = Some(after.into());
        self
    }

    /// Optional filter for fetching attestations with a given predicate type.
    pub fn predicate_type(mut self, predicate_type: impl Into<String>) -> Self {
        self.body.predicate_type = Some(predicate_type.into());
        self
    }

    /// Sends the actual request.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#list-attestations-by-bulk-subject-digests)
    pub async fn send(self) -> crate::Result<BulkAttestationsResponse> {
        let route = format!("/{}/attestations/bulk-list", self.handler.user);
        let uri = self
            .handler
            .crab
            .parameterized_uri(route, Some(&self.query))?;
        self.handler
            .crab
            .post(uri.to_string(), Some(&self.body))
            .await
    }
}

/// Builder for [`UserAttestationsHandler::list`].
#[derive(serde::Serialize)]
pub struct ListUserAttestationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r UserHandler<'octo>,
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

impl<'octo, 'r> ListUserAttestationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r UserHandler<'octo>, subject_digest: String) -> Self {
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
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/users/attestations#list-attestations)
    pub async fn send(self) -> crate::Result<AttestationsResponse> {
        let route = format!(
            "/{}/attestations/{}",
            self.handler.user, self.subject_digest
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
