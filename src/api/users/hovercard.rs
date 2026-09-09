use crate::api::users::UserHandler;
use crate::models::Hovercard;

/// A builder pattern struct for getting a user's hovercard contextual information.
///
/// Created by [`UserHandler::hovercard`].
#[derive(serde::Serialize)]
pub struct HovercardBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_type: Option<crate::params::users::hovercard::SubjectType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_id: Option<String>,
}

impl<'octo, 'b> HovercardBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b UserHandler<'octo>) -> Self {
        Self {
            handler,
            subject_type: None,
            subject_id: None,
        }
    }

    /// Identifies which additional information you'd like to receive about the person's hovercard.
    /// Can be `organization`, `repository`, `issue`, or `pull_request`.
    /// Required when using `subject_id`.
    pub fn with_subject_type(
        mut self,
        subject_type: impl Into<crate::params::users::hovercard::SubjectType>,
    ) -> Self {
        self.subject_type = Some(subject_type.into());
        self
    }

    /// Uses the ID for the `subject_type` you specified.
    /// Required when using `subject_type`.
    pub fn with_subject_id(mut self, subject_id: impl Into<String>) -> Self {
        self.subject_id = Some(subject_id.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Hovercard> {
        let route = format!("/{}/hovercard", self.handler.user);
        self.handler.crab.get(route, Some(&self)).await
    }
}
