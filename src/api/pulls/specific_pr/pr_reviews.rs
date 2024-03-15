use crate::pulls::specific_pr::pr_reviews::specific_review::SpecificReviewBuilder;
use crate::pulls::PullRequestHandler;

mod specific_review;

#[derive(serde::Serialize)]
pub struct ReviewsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    pr_number: u64,
}

impl<'octo, 'b> ReviewsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            pr_number,
        }
    }

    pub fn review(&self, review_id: u64) -> SpecificReviewBuilder<'octo, '_> {
        SpecificReviewBuilder::new(self.handler, self.pr_number, review_id)
    }
}
