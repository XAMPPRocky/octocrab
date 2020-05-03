use octocrab::{params, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::default();

    let mut current_page = octocrab
        .pulls("rust-lang", "rust")
        .list()
        .state(params::State::Open)
        .per_page(100)
        .send()
        .await?;
    let mut prs = current_page.take_items();

    while let Ok(Some(new_page)) = octocrab.get_page(&current_page.next).await {
        current_page = new_page;

        prs.extend(current_page.items);

        if prs.len() > 100 {
            println!("Got the first ~100 PRs stopping.");
            break;
        }
    }

    println!("Found {} total items", prs.len());

    for pr in prs {
        println!("{:?}", pr);
    }

    Ok(())
}
