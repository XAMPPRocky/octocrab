use octocrab::{models, pulls, Octocrab, Page};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::default();

    let mut current_page = octocrab
        .pulls("rust-lang", "rust")
        .list()
        .state(pulls::PullRequestState::Open)
        .send()
        .await?;
    let mut prs = current_page.take_items();

    while let Ok(Some(new_page)) = octocrab.get_page(&current_page.next).await {
        current_page = new_page;

        prs.extend(current_page.items);

        if prs.len() > 100 {
            println!("Got the first ~100 PRs.");
            break;
        }
    }

    println!("Found {} items", prs.len());

    Ok(())
}
