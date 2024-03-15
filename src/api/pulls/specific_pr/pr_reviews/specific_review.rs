use crate::pulls::PullRequestHandler;

#[derive(serde::Serialize)]
pub struct SpecificReviewBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    pr_number: u64,
    review_id: u64,
}

impl<'octo, 'b> SpecificReviewBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b PullRequestHandler<'octo>,
        pr_number: u64,
        review_id: u64,
    ) -> Self {
        Self {
            handler,
            pr_number,
            review_id,
        }
    }

    ///Retrieves a pull request review by its ID.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#get-a-review-for-a-pull-request
    pub async fn get(&self) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }

    ///Updates the contents of a specified review summary comment.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#update-a-review-for-a-pull-request
    pub async fn update(
        &self,
        body: impl Into<String>,
    ) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.patch(route, Some(&body.into())).await
    }

    ///Deletes a pull request review that has not been submitted. Submitted reviews cannot be deleted.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#delete-a-pending-review-for-a-pull-request
    pub async fn delete_pending(&self) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.delete(route, None::<&()>).await
    }
}
