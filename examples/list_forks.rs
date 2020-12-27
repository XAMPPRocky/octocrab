use octocrab::{repos::forks::ForkSort, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let forks = octocrab
        .repos("rust-lang", "rust")
        .list_forks()
        .sort(ForkSort::Oldest)
        .send()
        .await?;

    for f in forks {
        println!("fork: {}", f.owner.login);
    }
    Ok(())
}
