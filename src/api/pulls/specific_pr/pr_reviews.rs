use crate::pulls::specific_pr::pr_reviews::specific_review::SpecificReviewBuilder;
use crate::pulls::PullRequestHandler;

pub mod specific_review;

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

    /// Creates a new `SpecificReviewBuilder`
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .get() // + update, delete_pending, submit, dismiss, list_comments
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn review(&self, review_id: u64) -> SpecificReviewBuilder<'octo, '_> {
        SpecificReviewBuilder::new(self.handler, self.pr_number, review_id)
    }
}
